# `ollama-example`

- To get this running, I had to:
  - Add an override to `wasmcloud.toml`:
    ```toml
    [overrides]
    "thomastaylor312:ollama" = { path = "../wit/" }
    ```
  - Add the custom interface to `wit/world.wit` (i.e., `import thomastaylor312:ollama/generate;`)
  - Add a dependency to `wit-bindgen`
  - Add a call to `wit_bindgen::generate!({ generate_all });` in `src/lib.rs`—after that, you can just call into the interface like normal (i.e., `use thomastaylor312::ollama::generate::{generate, Request};`)
  - Add the custom capability provider to the `wadm.yaml`:
  ```shell
    - name: ollama
      type: capability
      properties:
        image: file://../build/ollama-provider.par.gz
  ```
  - Add link to the custom capability provider to the `wadm.yaml`:
  ```shell
        - type: link
          properties:
            target: ollama
            namespace: thomastaylor312
            package: ollama
            interfaces: [generate]
            target_config:
              - name: ollama-conf
                properties:
                  model_name: gurubot/tinystories-656k-q8    
  ```