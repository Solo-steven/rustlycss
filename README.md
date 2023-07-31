# rustlycss

rustlycss is a css transform tool builded in rust.Insipred by postcss, rustlycss accepts a relative loss syntax of css, provider developer write their own syntax transform to control css syntax.

## Benchmark

benchmark run on personal computed (Macbook pro 2021, M1 pro chip), tool using tinybench for js and criterion for rust.
### Parser
| **file** |  bootstrap-reboot.css   |    bootstrap.css     | tailwind-dark.css |
| -------: | ----------------------: | -------------------: | -----------------: | 
| **size** |                   7.4K  |               201K   |              5.8M   |
|   **js** |               339.410us |          7864.636 us |        238.186616ms |
| **rust** |                33.769us |            826.08 us |          24.122ms   |
|    **~** |                  10.05x|                9.5x  |                9.87x  |

### Codegen
| **file** |  bootstrap-reboot.css   |    bootstrap.css     | tailwind-dark.css   |
| -------: | ----------------------: | -------------------: | -----------------:  | 
| **size** |                   7.4K  |               201K   |              5.8M   |
|   **js** |               30.297us |          967.256 us |        47.220991ms    |
| **rust** |                3.4377us|            133.63 us |          4.2332ms    |
|    **~** |                  8.81x|                7.24x  |            11.15x    |

### Codegen With Source Map

| **file** |  bootstrap-reboot.css   |    bootstrap.css     | tailwind-dark.css   |
| -------: | ----------------------: | -------------------: | -----------------:  | 
| **size** |                   7.4K  |               201K   |              5.8M   |
|   **js** |               433.094us |         10102.929 us |        273.264262 ms |
| **rust** |                9.779us  |            319.47 us |          12.292ms    |
|    **~** |                 44.28x  |              31.624x |               22.23x |

## Syntax

Please to go `/spec/spec.pdf` for syntax production rule

## ACKNOWLEDGEMENT

This project is deeply inspired by postcss and postcss-rs, thanks [Andrey Sitnik](https://github.com/ai) and [postcss-rs contributor](https://github.com/postcss-rs/postcss-rs/graphs/contributors).