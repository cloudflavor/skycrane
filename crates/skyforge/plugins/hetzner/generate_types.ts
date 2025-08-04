#!/usr/bin/env -S deno run --allow-read --allow-write
/**
 * Generate Starlark type definitions from OpenAPI spec
 */

import * as fs from "node:fs";

interface OpenAPISpec {
  paths: Record<string, any>;
}

function toStarlarkType(schema: any): string {
  const type = schema.type;
  const required = schema.required || false;
  
  // Handle enums
  if (schema.enum) {
    const values = schema.enum.map((v: any) => `"${v}"`).join(", ");
    return required ? `Enum([${values}], required = True)` : `Enum([${values}])`;
  }
  
  // Handle basic types
  switch (type) {
    case "string":
      const parts = [];
      if (required) parts.push("required = True");
      if (schema.pattern) parts.push(`pattern = r"${schema.pattern}"`);
      if (schema.maxLength) parts.push(`max_length = ${schema.maxLength}`);
      if (schema.minLength) parts.push(`min_length = ${schema.minLength}`);
      if (schema.default) parts.push(`default = "${schema.default}"`);
      if (schema.example && !schema.default) parts.push(`default = "${schema.example}"`);
      return parts.length > 0 ? `String(${parts.join(", ")})` : "String()";
      
    case "integer":
    case "number":
      const numParts = [];
      if (required) numParts.push("required = True");
      if (schema.minimum !== undefined) numParts.push(`min = ${schema.minimum}`);
      if (schema.maximum !== undefined) numParts.push(`max = ${schema.maximum}`);
      if (schema.default !== undefined) numParts.push(`default = ${schema.default}`);
      return numParts.length > 0 ? `Int(${numParts.join(", ")})` : "Int()";
      
    case "boolean":
      return schema.default !== undefined ? `Bool(default = ${schema.default})` : "Bool()";
      
    case "array":
      if (schema.items) {
        const itemType = toStarlarkType(schema.items);
        return `List(${itemType})`;
      }
      return "List()";
      
    case "object":
      // For labels/tags
      return "Map(String(), String())";
      
    default:
      return "Any()";
  }
}

function extractResourceName(path: string): string {
  // /servers -> server
  // /ssh_keys -> ssh_key
  const parts = path.split("/").filter(p => p && !p.includes("{"));
  const name = parts[parts.length - 1];
  
  // Remove trailing 's' for plural
  if (name.endsWith("ies")) {
    return name.slice(0, -3) + "y";
  } else if (name.endsWith("s")) {
    return name.slice(0, -1);
  }
  return name;
}

function detectRelationships(resourceName: string, schema: any): string[] {
  const relationships = [];
  
  // Detect parent from field names
  for (const fieldName of Object.keys(schema.properties || {})) {
    if (fieldName === "server" || fieldName === "server_id") {
      relationships.push(`    _parent = "server",`);
    }
    if (fieldName === "network" || fieldName === "network_id") {
      relationships.push(`    _dependencies = ["network"],`);
    }
  }
  
  // Known parent-child relationships for Hetzner
  const childMap: Record<string, string[]> = {
    "server": ["volume", "floating_ip"],
    "network": ["subnet", "route"],
    "load_balancer": ["target", "service"],
    "firewall": ["rule"],
  };
  
  if (childMap[resourceName]) {
    relationships.push(`    _children = [${childMap[resourceName].map(c => `"${c}"`).join(", ")}],`);
  }
  
  return relationships;
}

function generateStarlarkTypes(spec: OpenAPISpec): string {
  let output = `# Auto-generated Starlark type definitions from Hetzner Cloud API
# Generated from: api/cloud.spec.json

# Type constructors
String = provider.String
Int = provider.Int
Bool = provider.Bool
Enum = provider.Enum
List = provider.List
Map = provider.Map
Ref = provider.Ref
Any = provider.Any

# Resource type definitions
resources = struct(\n`;
  
  // Get ALL resources with POST operations (create endpoints)
  const resourcePaths: string[] = [];
  for (const [path, methods] of Object.entries(spec.paths)) {
    if (methods.post && !path.includes("{") && path.split("/").length === 2) {
      resourcePaths.push(path);
    }
  }
  
  for (const path of resourcePaths) {
    if (!spec.paths[path]?.post?.requestBody) continue;
    
    const resourceName = extractResourceName(path);
    const schema = spec.paths[path].post.requestBody.content["application/json"].schema;
    const properties = schema.properties || {};
    const required = schema.required || [];
    
    output += `    ${resourceName} = struct(\n`;
    
    // Add fields
    for (const [fieldName, fieldSchema] of Object.entries(properties)) {
      // Skip certain fields
      if (fieldName === "labels" || fieldName === "user_data") continue;
      
      const fieldDef = { ...fieldSchema, required: required.includes(fieldName) };
      const starlarkType = toStarlarkType(fieldDef);
      
      output += `        ${fieldName} = ${starlarkType},\n`;
    }
    
    // Add labels if not SSH key or certificate
    if (!["ssh_key", "certificate"].includes(resourceName)) {
      output += `        labels = Map(String(), String(), default = {}),\n`;
    }
    
    // Add relationships
    const relationships = detectRelationships(resourceName, schema);
    for (const rel of relationships) {
      output += rel + "\n";
    }
    
    output += `    ),\n\n`;
  }
  
  output += `)`;
  
  return output;
}

// Main
if (import.meta.main) {
  const specContent = fs.readFileSync("api/cloud.spec.json", "utf8");
  const spec: OpenAPISpec = JSON.parse(specContent);
  
  const types = generateStarlarkTypes(spec);
  fs.writeFileSync("spec/types.star", types);
  
  console.log("Generated spec/types.star");
}