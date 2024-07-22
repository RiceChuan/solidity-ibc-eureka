// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.25;

import { IICS02ClientMsgs } from "./IICS02ClientMsgs.sol";

interface ILightClientMsgs {
    /// @notice The key-value pair.
    struct KVPair {
        bytes path;
        bytes value;
    }

    // @notice Initializes the light client with a trusted client state and consensus state
    // @dev Should be used in the constructor of the light client contract
    struct MsgInitialize {
        /// Initial client state
        bytes clientState;
        /// Initial consensus state
        bytes consensusState;
    }

    // @notice Message for querying the (non)membership of the key-value pairs in the Merkle root at a given height.
    // @dev If a value is empty, then we are querying for non-membership.
    // @dev The proof may differ depending on the client implementation and whether
    // `batchVerifyMembership` or `updateClientAndBatchVerifyMembership` is called.
    struct MsgBatchMembership {
        /// The proof
        bytes proof;
        /// Height of the proof
        IICS02ClientMsgs.Height proofHeight;
        /// The key-value pairs
        KVPair[] keyValues;
    }

    /// The result of an update operation
    enum UpdateResult {
        /// The update was successful
        Update,
        /// A misbehaviour was detected
        Misbehaviour,
        /// Client is already up to date
        NoOp
    }
}