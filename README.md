# Ollama Capability Provider

This capability provider is an implementation of the `thomastaylor312:ollama` interface. It exposes
the Ollama API to components.

## Link Configuration

To configure this provider, use the following configuration values as `target_config` in the link:

| Property     | Description                                                                          |
| :----------- | :----------------------------------------------------------------------------------- |
| `model_name` | The name of the model to use for requests                                            |
| `url`        | The URL of the Ollama API. If not specified, the default is `http://localhost:11434` |

## Caveats

Currently, wasmCloud doesn't support resources in custom interfaces. The support for doing this just
landed in upstream wasmtime and should be added soon, which will make this interface better.

## Example

To run the example, make sure you have ollama installed locally and running (either as an
application or using `ollama serve`) and have run `ollama pull gurubot/tinystories-656k-q8`. You can use
another model, but please make sure to update it in `example/wadm.yaml`. Then run the following
commands

```terminal
# In a separate terminal
wash up

# Build the provider
wash build

# Build the component
cd example
wash build

# Run the component
wash app deploy wadm.yaml
```
