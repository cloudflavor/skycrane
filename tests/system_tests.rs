use anyhow::Ok;
use once_cell::sync::OnceCell;

static INIT: OnceCell<()> = OnceCell::new();

fn setup() {}

#[test]
fn system_test_load_plugins() {
    INIT.get_or_try_init(|| {
        setup();
        Ok(())
    })
    .unwrap();
}
