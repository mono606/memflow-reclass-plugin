# memflow-reclass-plugin

This plugin integrates the [memflow](https://github.com/memflow/memflow) physical memory introspection framework with [ReClass.NET](https://github.com/ReClassNET/ReClass.NET).

The plugin uses the memflow crates internally and also holds caches locally. Any connector that can be used with memflow can also be used with this plugin.

## Compilation

Just run the following command to compile the plugin.

```
cargo build --release
```

The resulting plugin can be found under `./target/release/libmemflow_reclass.so` (or dll on windows).

## Usage

After the plugin has been copied to the `./Plugins` folder in ReClass it can be selected as a plugin inside of ReClass.

Make sure you start [ReClass.NET](https://github.com/ReClassNET/ReClass.NET) with the appropiate access rights (e.g. SYS_PTRACE) for the connector you intend to use.
**Note: Since ReClass.NET is a mono application the `mono` binary will need to have the SYS_PTRACE access rights!**

More information on access rights can be found in the [memflow repository](https://github.com/memflow/memflow) or in the specific connector repository.

## Configuration

The `memflow.toml` file is used to configure the plugin and configure the memflow connector that should be used. The file has the following format:
```toml
connector = "kvm" # the name of the connector to use
args = "" # the argument string passed to the connector, optional
log_level = "info" # changes the memflow logging level, optional
parse_sections = true # will load section information of the process
```

Depending on the Connector you use it might be useful to disable section parsing as this slow down the ReClass UI.

The plugin will try to read the config file from the current workdir + `/Plugins/`.

## Remarks

This plugin is still work in progress and some features might not yet work as expected.

## License

Licensed under MIT License, see [LICENSE](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
