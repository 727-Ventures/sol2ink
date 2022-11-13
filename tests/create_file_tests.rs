use sol_to_ink::run;

#[test]
fn erc20_creating() {
    assert_eq!(
        run(&"examples/contracts/ERC20/ERC20.sol".to_string()),
        Ok(())
    );
}

#[test]
fn erc721_creating() {
    assert_eq!(
        run(&"examples/contracts/ERC721/ERC721.sol".to_string()),
        Ok(())
    );
}

#[test]
fn erc1155_creating() {
    assert_eq!(
        run(&"examples/contracts/ERC1155/ERC1155.sol".to_string()),
        Ok(())
    );
}

#[test]
fn access_control_creating() {
    assert_eq!(
        run(&"examples/contracts/AccessControl/AccessControl.sol".to_string()),
        Ok(())
    );
}

#[test]
fn solang_example_creating() {
    assert_eq!(
        run(&"examples/contracts/SolangExample/example.sol".to_string()),
        Ok(())
    );
}

#[test]
fn flipper_creating() {
    assert_eq!(
        run(&"examples/contracts/Flipper/flipper.sol".to_string()),
        Ok(())
    );
}

#[test]
fn primitives_creating() {
    assert_eq!(
        run(&"examples/contracts/Primitives/Primitives.sol".to_string()),
        Ok(())
    );
}

#[test]
fn ierc20_creating() {
    assert_eq!(
        run(&"examples/interfaces/IERC20/IERC20.sol".to_string()),
        Ok(())
    );
}

#[test]
fn ierc721_creating() {
    assert_eq!(
        run(&"examples/interfaces/IERC721/IERC721.sol".to_string()),
        Ok(())
    );
}

#[test]
fn ierc1155_creating() {
    assert_eq!(
        run(&"examples/interfaces/IERC1155/IERC1155.sol".to_string()),
        Ok(())
    );
}

#[test]
fn iaccess_control_creating() {
    assert_eq!(
        run(&"examples/interfaces/IAccessControl/IAccessControl.sol".to_string()),
        Ok(())
    );
}
