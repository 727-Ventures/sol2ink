use sol_to_ink::run;
use std::fs;

#[test]
fn erc20_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/ERC20/ERC20/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/contracts/ERC20/ERC20.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/ERC20/ERC20/lib.rs").unwrap()
    );
}

#[test]
fn erc721_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/ERC721/ERC721/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/contracts/ERC721/ERC721.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/ERC721/ERC721/lib.rs").unwrap()
    );
}

#[test]
fn erc1155_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/ERC1155/ERC1155/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/contracts/ERC1155/ERC1155.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/ERC1155/ERC1155/lib.rs").unwrap()
    );
}

#[test]
fn access_control_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/AccessControl/AccessControl/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/contracts/AccessControl/AccessControl.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/AccessControl/AccessControl/lib.rs").unwrap()
    );
}

#[test]
fn solang_example_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/SolangExample/example/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/contracts/SolangExample/example.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/SolangExample/example/lib.rs").unwrap()
    );
}

#[test]
fn flipper_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/Flipper/flipper/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/contracts/Flipper/flipper.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/Flipper/flipper/lib.rs").unwrap()
    );
}

#[test]
fn primitives_is_not_changed() {
    let file = fs::read_to_string("examples/contracts/Primitives/Primitives/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/contracts/Primitives/Primitives.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/contracts/Primitives/Primitives/lib.rs").unwrap()
    );
}

#[test]
fn ierc20_is_not_changed() {
    let file = fs::read_to_string("examples/interfaces/IERC20/IERC20/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/interfaces/IERC20/IERC20.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/interfaces/IERC20/IERC20/lib.rs").unwrap()
    );
}

#[test]
fn ierc721_is_not_changed() {
    let file = fs::read_to_string("examples/interfaces/IERC721/IERC721/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/interfaces/IERC721/IERC721.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/interfaces/IERC721/IERC721/lib.rs").unwrap()
    );
}

#[test]
fn ierc1155_is_not_changed() {
    let file = fs::read_to_string("examples/interfaces/IERC1155/IERC1155/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/interfaces/IERC1155/IERC1155.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/interfaces/IERC1155/IERC1155/lib.rs").unwrap()
    );
}

#[test]
fn iaccess_control_is_not_changed() {
    let file =
        fs::read_to_string("examples/interfaces/IAccessControl/IAccessControl/lib.rs").unwrap();

    assert_eq!(
        run(&"examples/interfaces/IAccessControl/IAccessControl.sol".to_string()),
        Ok(())
    );
    assert_eq!(
        file,
        fs::read_to_string("examples/interfaces/IAccessControl/IAccessControl/lib.rs").unwrap()
    );
}
