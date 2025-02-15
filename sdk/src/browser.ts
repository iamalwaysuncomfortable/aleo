import "./polyfill/shared";

import { Account } from "./account";
import { AleoNetworkClient, ProgramImports } from "./network-client";
import { Block } from "./models/block";
import { Execution } from "./models/execution";
import { Input } from "./models/input";
import { Output } from "./models/output";
import { TransactionModel } from "./models/transactionModel";
import { Transition } from "./models/transition";
import {
    AleoKeyProvider,
    AleoKeyProviderParams,
    AleoKeyProviderInitParams,
    CachedKeyPair,
    FunctionKeyPair,
    FunctionKeyProvider,
    KeySearchParams,
} from "./function-key-provider";
import {
    OfflineKeyProvider,
    OfflineSearchParams
} from "./offline-key-provider";
import {
    BlockHeightSearch,
    NetworkRecordProvider,
    RecordProvider,
    RecordSearchParams,
} from "./record-provider";

// @TODO: This function is no longer needed, remove it.
async function initializeWasm() {
    console.warn("initializeWasm is deprecated, you no longer need to use it");
}

export { createAleoWorker } from "./managed-worker";

export { ProgramManager } from "./program-manager";

export { logAndThrow } from "./utils";

export {
    Address,
    Execution as FunctionExecution,
    ExecutionResponse,
    Field,
    OfflineQuery,
    PrivateKey,
    PrivateKeyCiphertext,
    Program,
    ProgramManager as ProgramManagerBase,
    ProvingKey,
    RecordCiphertext,
    RecordPlaintext,
    Signature,
    Transaction,
    VerifyingKey,
    ViewKey,
    initThreadPool,
    verifyFunctionExecution,
} from "./wasm";

export { initializeWasm };

export {
    Key,
    CREDITS_PROGRAM_KEYS,
    KEY_STORE,
    PRIVATE_TRANSFER,
    PRIVATE_TO_PUBLIC_TRANSFER,
    PRIVATE_TRANSFER_TYPES,
    PUBLIC_TRANSFER,
    PUBLIC_TRANSFER_AS_SIGNER,
    PUBLIC_TO_PRIVATE_TRANSFER,
    VALID_TRANSFER_TYPES,
} from "./constants";

export {
    Account,
    AleoKeyProvider,
    AleoKeyProviderParams,
    AleoKeyProviderInitParams,
    AleoNetworkClient,
    Block,
    BlockHeightSearch,
    CachedKeyPair,
    Execution,
    FunctionKeyPair,
    FunctionKeyProvider,
    Input,
    KeySearchParams,
    NetworkRecordProvider,
    ProgramImports,
    OfflineKeyProvider,
    OfflineSearchParams,
    Output,
    RecordProvider,
    RecordSearchParams,
    TransactionModel,
    Transition,
};
