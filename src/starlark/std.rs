use allocative::Allocative;
use anyhow::Context;
use starlark::environment::GlobalsBuilder;
use starlark::values::AllocValue;
use starlark::values::{Heap, NoSerialize, ProvidesStaticType, StarlarkValue, Value, ValueLike};
use starlark::{starlark_module, starlark_simple_value};
use starlark_derive::starlark_value;
use std::fmt;

pub const INHERIT_ARGS: &str = "InheritArgs";
pub const INHERIT_ENV: &str = "InheritEnv";
pub const INHERIT_STDIN: &str = "InheritStdin";
pub const INHERIT_STDIO: &str = "InheritStdio";
pub const INHERIT_STDOUT: &str = "InheritStdout";
pub const INHERIT_NETWORK: &str = "InheritNetwork";
pub const ALLOW_IP_NAME_LOOKUP: &str = "AllowIpNameLookup";

#[starlark_module]
pub fn skycrane_std(builder: &mut GlobalsBuilder) {
    // These need to be defined in the module so that they are part ot the starlark std
    // an Enum has been considered, but it's less feasible since they need to be allocated
    // to the starlark heap via functions and instead of INHERIT_ARGS in starlark, it
    // would be INHERIT_ARGS().
    const INHERIT_ARGS: &str = INHERIT_ARGS;
    const INHERIT_ENV: &str = INHERIT_ENV;
    const INHErit_STDIN: &str = INHERIT_STDIN;
    const INHERIT_STDIO: &str = INHERIT_STDIO;
    const INHERIT_STDOUT: &str = INHERIT_STDOUT;
    const INHERIT_NETWORK: &str = INHERIT_NETWORK;
    const ALLOW_IP_NAME_LOOKUP: &str = ALLOW_IP_NAME_LOOKUP;

    fn mount<'v>(
        host_path: String,
        guest_path: String,
        file_perms: Option<Value<'v>>,
        dir_perms: Option<Value<'v>>,
    ) -> anyhow::Result<PluginMount> {
        let file_perms = if let Some(file_perms) = file_perms {
            file_perms
                .downcast_ref::<FilePerms>()
                .with_context(|| "failed to interpret file permissions")?
                .to_owned()
        } else {
            FilePerms {
                read: false,
                write: false,
            }
        };

        let dir_perms = if let Some(dir_perms) = dir_perms {
            dir_perms
                .downcast_ref::<DirPerms>()
                .with_context(|| "failed to interpret directory permissions")?
                .to_owned()
        } else {
            DirPerms {
                read: false,
                mutate: false,
            }
        };

        Ok(PluginMount {
            host_path,
            guest_path,
            file_perms,
            dir_perms,
        })
    }

    fn capabilities<'v>(
        inherits: Value<'v>,
        mounts: Value<'v>,
        heap: &'v Heap,
    ) -> anyhow::Result<PluginCapabilities> {
        let mut inherited_caps = Vec::new();
        let mut mount_results = Vec::new();

        for inherit in inherits.iterate(heap).unwrap() {
            match inherit.to_str().as_str() {
                INHERIT_ARGS => inherited_caps.push(INHERIT_ARGS),
                INHERIT_ENV => inherited_caps.push(INHERIT_ENV),
                INHERIT_STDIN => inherited_caps.push(INHERIT_STDIN),
                INHERIT_STDIO => inherited_caps.push(INHERIT_STDIO),
                INHERIT_STDOUT => inherited_caps.push(INHERIT_STDOUT),
                INHERIT_NETWORK => inherited_caps.push(INHERIT_NETWORK),
                ALLOW_IP_NAME_LOOKUP => inherited_caps.push(ALLOW_IP_NAME_LOOKUP),
                _ => anyhow::bail!("Invalid capability"),
            }
        }

        for mount in mounts.iterate(heap).unwrap() {
            let a = mount
                .downcast_ref::<PluginMount>()
                .with_context(|| "failed to interpret mount declaration")?;
            mount_results.push(a.to_owned());
        }

        Ok(PluginCapabilities {
            inherits: Vec::new(),
            mounts: Vec::new(),
        })
    }

    fn module<'v>(
        name: String,
        version: String,
        capabilities: Option<Value<'v>>,
        mounts: Option<Value<'v>>,
        heap: &'v Heap,
    ) -> anyhow::Result<CloudModule> {
        if let Some(mounts) = mounts {
            for mount in mounts.iterate(heap).unwrap() {
                let a = mount
                    .downcast_ref::<PluginMount>()
                    .with_context(|| "failed to interpret mount declaration")?;
            }
        }

        if let Some(caps) = capabilities {
            for item in caps.iterate(heap).unwrap() {}
        }

        Ok(CloudModule {
            name,
            version,
            capabilities: Vec::new(),
            mounts: Vec::new(),
        })
    }
}

fn extract_file_perms<'v>(value: Option<Value<'v>>) -> anyhow::Result<FilePerms> {
    match value {
        Some(v) => v
            .downcast_ref::<FilePerms>()
            .cloned()
            .context("failed to interpret file permissions"),
        None => Ok(FilePerms {
            read: false,
            write: false,
        }),
    }
}

fn extract_dir_perms<'v>(value: Option<Value<'v>>) -> anyhow::Result<DirPerms> {
    match value {
        Some(v) => v
            .downcast_ref::<DirPerms>()
            .cloned()
            .context("failed to interpret directory permissions"),
        None => Ok(DirPerms {
            read: false,
            mutate: false,
        }),
    }
}

fn parse_inherits<'v>(inherits: Value<'v>, heap: &'v Heap) -> anyhow::Result<Vec<String>> {
    let mut inherited_caps = Vec::new();
    match inherits.iterate(heap) {
        Ok(inherited) => {
            for inherit in inherited {
                match inherit.to_str().as_str() {
                    INHERIT_ARGS => inherited_caps.push(INHERIT_ARGS.to_string()),
                    INHERIT_ENV => inherited_caps.push(INHERIT_ENV.to_string()),
                    INHERIT_STDIN => inherited_caps.push(INHERIT_STDIN.to_string()),
                    INHERIT_STDIO => inherited_caps.push(INHERIT_STDIO.to_string()),
                    INHERIT_STDOUT => inherited_caps.push(INHERIT_STDOUT.to_string()),
                    INHERIT_NETWORK => inherited_caps.push(INHERIT_NETWORK.to_string()),
                    ALLOW_IP_NAME_LOOKUP => inherited_caps.push(ALLOW_IP_NAME_LOOKUP.to_string()),
                    _ => anyhow::bail!("Invalid capability"),
                }
            }

            Ok(inherited_caps)
        }
        Err(err) => anyhow::bail!("Failed to iterate over inherits: {err}"),
    }
}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct PluginCapabilities {
    inherits: Vec<String>,
    mounts: Vec<PluginMount>,
}

impl<'v> AllocValue<'v> for PluginCapabilities {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc(self)
    }
}

impl fmt::Display for PluginCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cap in self.inherits.iter() {
            write!(f, "inherits: {cap}");
        }
        for mount in self.mounts.iter() {
            write!(f, "mounts: {mount}");
        }
        Ok(())
    }
}

#[starlark_value(type = "plugin_caps")]
impl<'v> StarlarkValue<'v> for PluginCapabilities {}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct PluginMount {
    pub host_path: String,
    pub guest_path: String,
    pub file_perms: FilePerms,
    pub dir_perms: DirPerms,
}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct FilePerms {
    pub read: bool,
    pub write: bool,
}

impl fmt::Display for FilePerms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "read: {}, write: {}", self.read, self.write)
    }
}

#[starlark_value(type = "file_perms")]
impl<'v> StarlarkValue<'v> for FilePerms {}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct DirPerms {
    pub read: bool,
    pub mutate: bool,
}

impl fmt::Display for DirPerms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "read: {}, mutate: {}", self.read, self.mutate)
    }
}

#[starlark_value(type = "dir_perms")]
impl<'v> StarlarkValue<'v> for DirPerms {}

impl<'v> AllocValue<'v> for PluginMount {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc(self)
    }
}

impl fmt::Display for PluginMount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host_path: {}, guest_path: {}",
            self.host_path, self.guest_path
        )
    }
}

#[starlark_value(type = "mount_path")]
impl<'v> StarlarkValue<'v> for PluginMount {}

#[derive(Debug, PartialEq, Eq, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct CloudModule {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub mounts: Vec<PluginMount>,
}
starlark_simple_value!(CloudModule);

#[starlark_value(type = "cloud_module")]
impl<'v> StarlarkValue<'v> for CloudModule {}

impl fmt::Display for CloudModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}, version: {}", self.name, self.version)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_iterate_over_inherits() {
        let heap = Heap::new();

        let inherits_list = vec![
            heap.alloc(INHERIT_ARGS),
            heap.alloc(INHERIT_STDIN),
            heap.alloc(INHERIT_STDIO),
            heap.alloc(INHERIT_STDOUT),
            heap.alloc(INHERIT_NETWORK),
            heap.alloc(ALLOW_IP_NAME_LOOKUP),
        ];
        let inherits = heap.alloc(inherits_list);

        let result = parse_inherits(inherits, &heap).unwrap();
        assert_eq!(result.len(), 7);
    }
}
