// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface ICivilizations {
    function setInitialized() external;

    function setPrice(uint256 _price) external;

    function setPaymentsReceiver(address payable _payments_receiver) external;

    function addCivilization(address _instance) external;

    function mint(address _instance) external payable;

    function getID(address _instance) external view returns (uint256);

    function getCivilizations() external view returns (address[] memory);

    function getTokenID(address _instance, uint256 _id)
        external
        view
        returns (bytes memory);

    function isAllowed(address spender, bytes memory _id)
        external
        view
        returns (bool);

    function exists(bytes memory _id) external view returns (bool);
}
