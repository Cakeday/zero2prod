Use bunyan to output prettified logs from tests:
```sh
TEST_LOG=true cargo test health_check_works | bunyan
```