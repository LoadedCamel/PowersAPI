# PowersAPI
A modified version of RubyRed's Powers API, found here: https://coh.tips/powers/

---

## Parser for various powers binary data

This parser can specifically read from a few CoH client .bin files to try and assemble the tree of archetypes, powers,
and power sets. It then reduces the relevant static information down to a set of JSON files that can be used locally or
as part of an online "API".

The live version is here: https://coh.tips/powers/

While this project is written in Rust, I avoided getting too idiomatic with the codebase. It's very verbose and 
thoroughly commented, so it should serve as a good map if you're looking at how the bin files are structured, even
if you don't speak Rust.

For more information about bin parsing, check my blog post: [Code archaeology: Reading City of Heroes' .bin files](https://rubidium.dev/2020/03/07/code-archaeology-reading-city-of-heroes-bin-files.html)

## Building

You will need latest version of Rust and the bin files you want to parse. Note that the bin files are a moving target,
so depending on which version you have, they may not work with this parser.

I don't extract from piggs, as there's plenty of adequate tools for that. You'll need to extract:

* `clientmessages-en.bin`
* `attrib_names.bin`
* `boostsets.bin`
* `classes.bin`
* `powercats.bin`
* `powersets.bin`
* `powers.bin`
* `villain_classes.bin`
* `VillainDef.bin`

Refer to [PowersConfig.toml](PowersConfig.toml) for how to configure.

Once you have everything set, simply run:

```cargo run --release```

And this will do the magic. I recommend release mode while you're not debugging as it parses much faster.

**Note:** Version 2.0.0 forward require a nightly version of Rust for the time being.

## Output

The description of the JSON output files can be found in the [data dictionary](docs/index.md).

## License

The application is distributed under an MIT license. You're welcome to copy, modify, and set up your own site if you want, as long as you follow the rules of the license. Refer to the [license file](LICENSE.md) for more information.