# Auto-generated Starlark type definitions from Hetzner Cloud API
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
resources = struct(
    certificate = struct(
        name = String(required = True, default = "my website cert"),
        type = Enum(["uploaded", "managed"]),
        certificate = String(default = "-----BEGIN CERTIFICATE-----
..."),
        private_key = String(default = "-----BEGIN PRIVATE KEY-----
..."),
        domain_names = List(String()),
    ),

    firewall = struct(
        name = String(required = True, default = "new-name"),
        rules = List(Map(String(), String())),
        apply_to = List(Map(String(), String())),
        labels = Map(String(), String(), default = {}),
    _children = ["rule"],
    ),

    floating_ip = struct(
        type = Enum(["ipv4", "ipv6"], required = True),
        server = Int(),
        home_location = String(default = "fsn1"),
        description = String(default = "This describes my resource"),
        name = String(default = "my-resource"),
        labels = Map(String(), String(), default = {}),
    _parent = "server",
    ),

    load_balancer = struct(
        name = String(required = True, default = "Web Frontend"),
        load_balancer_type = String(required = True, default = "lb11"),
        algorithm = Map(String(), String()),
        services = List(Map(String(), String())),
        targets = List(Map(String(), String())),
        public_interface = Bool(),
        network = Int(),
        network_zone = String(default = "eu-central"),
        location = String(default = "fsn1"),
        labels = Map(String(), String(), default = {}),
    _dependencies = ["network"],
    _children = ["target", "service"],
    ),

    network = struct(
        name = String(required = True, default = "mynet"),
        ip_range = String(required = True, default = "10.0.0.0/16"),
        subnets = List(Map(String(), String())),
        routes = List(Map(String(), String())),
        expose_routes_to_vswitch = Bool(),
        labels = Map(String(), String(), default = {}),
    _children = ["subnet", "route"],
    ),

    placement_group = struct(
        name = String(required = True, default = "my Placement Group"),
        type = Enum(["spread"], required = True),
        labels = Map(String(), String(), default = {}),
    ),

    primary_ip = struct(
        name = String(required = True, default = "my-resource"),
        type = Enum(["ipv4", "ipv6"], required = True),
        datacenter = String(default = "fsn1-dc8"),
        assignee_type = Enum(["server"], required = True),
        assignee_id = Int(),
        auto_delete = Bool(default = false),
        labels = Map(String(), String(), default = {}),
    ),

    server = struct(
        name = String(required = True, default = "my-server"),
        location = String(default = "nbg1"),
        datacenter = String(default = "nbg1-dc3"),
        server_type = String(required = True, default = "cpx11"),
        start_after_create = Bool(default = true),
        image = String(required = True, default = "ubuntu-20.04"),
        placement_group = Int(),
        ssh_keys = List(String()),
        volumes = List(Int()),
        networks = List(Int()),
        firewalls = List(Map(String(), String())),
        automount = Bool(),
        public_net = Map(String(), String()),
        labels = Map(String(), String(), default = {}),
    _children = ["volume", "floating_ip"],
    ),

    ssh_key = struct(
        name = String(required = True, default = "My ssh key"),
        public_key = String(required = True, default = "ssh-rsa AAAjjk76kgf...Xt"),
    ),

    volume = struct(
        size = Int(required = True),
        name = String(required = True, default = "databases-storage"),
        automount = Bool(),
        format = String(default = "xfs"),
        location = String(default = "nbg1"),
        server = Int(),
        labels = Map(String(), String(), default = {}),
    _parent = "server",
    ),
)
