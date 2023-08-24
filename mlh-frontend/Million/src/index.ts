import * as SorobanClient from 'soroban-client';
import { xdr } from 'soroban-client';
import { Buffer } from "buffer";
import { scValStrToJs, scValToJs, addressToScVal, u128ToScVal, i128ToScVal, strToScVal } from './convert.js';
import { invoke } from './invoke.js';
import type { ResponseTypes, Wallet } from './method-options.js'

export * from './constants.js'
export * from './server.js'
export * from './invoke.js'

export type u32 = number;
export type i32 = number;
export type u64 = bigint;
export type i64 = bigint;
export type u128 = bigint;
export type i128 = bigint;
export type u256 = bigint;
export type i256 = bigint;
export type Address = string;
export type Option<T> = T | undefined;
export type Typepoint = bigint;
export type Duration = bigint;

/// Error interface containing the error message
export interface Error_ { message: string };

export interface Result<T, E = Error_> {
    unwrap(): T,
    unwrapErr(): E,
    isOk(): boolean,
    isErr(): boolean,
};

export class Ok<T> implements Result<T> {
    constructor(readonly value: T) { }
    unwrapErr(): Error_ {
        throw new Error('No error');
    }
    unwrap(): T {
        return this.value;
    }

    isOk(): boolean {
        return true;
    }

    isErr(): boolean {
        return !this.isOk()
    }
}

export class Err<T> implements Result<T> {
    constructor(readonly error: Error_) { }
    unwrapErr(): Error_ {
        return this.error;
    }
    unwrap(): never {
        throw new Error(this.error.message);
    }

    isOk(): boolean {
        return false;
    }

    isErr(): boolean {
        return !this.isOk()
    }
}

if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
}

const regex = /ContractError\((\d+)\)/;

function getError(err: string): Err<Error_> | undefined {
    const match = err.match(regex);
    if (!match) {
        return undefined;
    }
    if (Errors == undefined) {
        return undefined;
    }
    // @ts-ignore
    let i = parseInt(match[1], 10);
    if (i < Errors.length) {
        return new Err(Errors[i]!);
    }
    return undefined;
}

export type MillionDataKey = {tag: "TokenId", values: void} | {tag: "AssetAddress", values: void} | {tag: "Price", values: void};

function MillionDataKeyToXdr(millionDataKey?: MillionDataKey): xdr.ScVal {
    if (!millionDataKey) {
        return xdr.ScVal.scvVoid();
    }
    let res: xdr.ScVal[] = [];
    switch (millionDataKey.tag) {
        case "TokenId":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("TokenId"));
            break;
    case "AssetAddress":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("AssetAddress"));
            break;
    case "Price":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Price"));
            break;  
    }
    return xdr.ScVal.scvVec(res);
}

function MillionDataKeyFromXdr(base64Xdr: string): MillionDataKey {
    type Tag = MillionDataKey["tag"];
    type Value = MillionDataKey["values"];
    let [tag, values] = strToScVal(base64Xdr).vec()!.map(scValToJs) as [Tag, Value];
    if (!tag) {
        throw new Error('Missing enum tag when decoding MillionDataKey from XDR');
    }
    return { tag, values } as MillionDataKey;
}

export type Coords = {tag: "Token", values: [u32, u32]} | {tag: "Xy", values: [u32]};

function CoordsToXdr(coords?: Coords): xdr.ScVal {
    if (!coords) {
        return xdr.ScVal.scvVoid();
    }
    let res: xdr.ScVal[] = [];
    switch (coords.tag) {
        case "Token":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Token"));
            res.push(((i)=>xdr.ScVal.scvU32(i))(coords.values[0]));
            res.push(((i)=>xdr.ScVal.scvU32(i))(coords.values[1]));
            break;
    case "Xy":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Xy"));
            res.push(((i)=>xdr.ScVal.scvU32(i))(coords.values[0]));
            break;  
    }
    return xdr.ScVal.scvVec(res);
}

function CoordsFromXdr(base64Xdr: string): Coords {
    type Tag = Coords["tag"];
    type Value = Coords["values"];
    let [tag, values] = strToScVal(base64Xdr).vec()!.map(scValToJs) as [Tag, Value];
    if (!tag) {
        throw new Error('Missing enum tag when decoding Coords from XDR');
    }
    return { tag, values } as Coords;
}

export async function upgrade<R extends ResponseTypes = undefined>({wasm_hash}: {wasm_hash: Buffer}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `void`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'upgrade',
        args: [((i) => xdr.ScVal.scvBytes(i))(wasm_hash)],
        ...options,
        parseResultXdr: () => {},
    });
}

export async function mint<R extends ResponseTypes = undefined>({x, y, to}: {x: u32, y: u32, to: Address}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `Ok<u32> | Err<Error_> | undefined`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'mint',
        args: [((i) => xdr.ScVal.scvU32(i))(x),
        ((i) => xdr.ScVal.scvU32(i))(y),
        ((i) => addressToScVal(i))(to)],
        ...options,
        parseResultXdr: (xdr): Ok<u32> | Err<Error_> | undefined => {
            try {
                return new Ok(scValStrToJs(xdr));
            } catch (e) {
                //@ts-ignore
                let err = getError(e.message);
                if (err) {
                    return err;
                } else {
                    throw e;
                }
            }
        },
    });
}

export async function balanceOf<R extends ResponseTypes = undefined>({owner}: {owner: Address}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `u32`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'balance_of',
        args: [((i) => addressToScVal(i))(owner)],
        ...options,
        parseResultXdr: (xdr): u32 => {
            return scValStrToJs(xdr);
        },
    });
}

export async function transferFrom<R extends ResponseTypes = undefined>({spender, from, to, token_id}: {spender: Address, from: Address, to: Address, token_id: u32}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `void`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'transfer_from',
        args: [((i) => addressToScVal(i))(spender),
        ((i) => addressToScVal(i))(from),
        ((i) => addressToScVal(i))(to),
        ((i) => xdr.ScVal.scvU32(i))(token_id)],
        ...options,
        parseResultXdr: () => {},
    });
}

export async function approve<R extends ResponseTypes = undefined>({caller, operator, token_id, expiration_ledger}: {caller: Address, operator: Option<Address>, token_id: u32, expiration_ledger: u32}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `void`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'approve',
        args: [((i) => addressToScVal(i))(caller),
        ((i) => (!i) ? xdr.ScVal.scvVoid() : addressToScVal(i))(operator),
        ((i) => xdr.ScVal.scvU32(i))(token_id),
        ((i) => xdr.ScVal.scvU32(i))(expiration_ledger)],
        ...options,
        parseResultXdr: () => {},
    });
}

export async function setApprovalForAll<R extends ResponseTypes = undefined>({caller, owner, operator, approved, expiration_ledger}: {caller: Address, owner: Address, operator: Address, approved: boolean, expiration_ledger: u32}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `void`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'set_approval_for_all',
        args: [((i) => addressToScVal(i))(caller),
        ((i) => addressToScVal(i))(owner),
        ((i) => addressToScVal(i))(operator),
        ((i) => xdr.ScVal.scvBool(i))(approved),
        ((i) => xdr.ScVal.scvU32(i))(expiration_ledger)],
        ...options,
        parseResultXdr: () => {},
    });
}

export async function getApproved<R extends ResponseTypes = undefined>({token_id}: {token_id: u32}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `Option<Address>`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'get_approved',
        args: [((i) => xdr.ScVal.scvU32(i))(token_id)],
        ...options,
        parseResultXdr: (xdr): Option<Address> => {
            return scValStrToJs(xdr);
        },
    });
}

export async function isApprovalForAll<R extends ResponseTypes = undefined>({owner, operator}: {owner: Address, operator: Address}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `boolean`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'is_approval_for_all',
        args: [((i) => addressToScVal(i))(owner),
        ((i) => addressToScVal(i))(operator)],
        ...options,
        parseResultXdr: (xdr): boolean => {
            return scValStrToJs(xdr);
        },
    });
}

export async function name<R extends ResponseTypes = undefined>(options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `string`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'name',
        ...options,
        parseResultXdr: (xdr): string => {
            return scValStrToJs(xdr);
        },
    });
}

export async function symbol<R extends ResponseTypes = undefined>(options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `string`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'symbol',
        ...options,
        parseResultXdr: (xdr): string => {
            return scValStrToJs(xdr);
        },
    });
}

export async function tokenUri<R extends ResponseTypes = undefined>({token_id}: {token_id: u32}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `string`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'token_uri',
        args: [((i) => xdr.ScVal.scvU32(i))(token_id)],
        ...options,
        parseResultXdr: (xdr): string => {
            return scValStrToJs(xdr);
        },
    });
}

export async function totalSupply<R extends ResponseTypes = undefined>(options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `u32`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'total_supply',
        ...options,
        parseResultXdr: (xdr): u32 => {
            return scValStrToJs(xdr);
        },
    });
}

export async function ownerOf<R extends ResponseTypes = undefined>({token_id}: {token_id: u32}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `Address`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'owner_of',
        args: [((i) => xdr.ScVal.scvU32(i))(token_id)],
        ...options,
        parseResultXdr: (xdr): Address => {
            return scValStrToJs(xdr);
        },
    });
}

export async function coords<R extends ResponseTypes = undefined>({token_id}: {token_id: u32}, options: {
  /**
   * The fee to pay for the transaction. Default: 100.
   */
  fee?: number
  /**
   * What type of response to return.
   *
   *   - `undefined`, the default, parses the returned XDR as `Option<[u32, u32]>`. Runs preflight, checks to see if auth/signing is required, and sends the transaction if so. If there's no error and `secondsToWait` is positive, awaits the finalized transaction.
   *   - `'simulated'` will only simulate/preflight the transaction, even if it's a change/set method that requires auth/signing. Returns full preflight info.
   *   - `'full'` return the full RPC response, meaning either 1. the preflight info, if it's a view/read method that doesn't require auth/signing, or 2. the `sendTransaction` response, if there's a problem with sending the transaction or if you set `secondsToWait` to 0, or 3. the `getTransaction` response, if it's a change method with no `sendTransaction` errors and a positive `secondsToWait`.
   */
  responseType?: R
  /**
   * If the simulation shows that this invocation requires auth/signing, `invoke` will wait `secondsToWait` seconds for the transaction to complete before giving up and returning the incomplete {@link SorobanClient.SorobanRpc.GetTransactionResponse} results (or attempting to parse their probably-missing XDR with `parseResultXdr`, depending on `responseType`). Set this to `0` to skip waiting altogether, which will return you {@link SorobanClient.SorobanRpc.SendTransactionResponse} more quickly, before the transaction has time to be included in the ledger. Default: 10.
   */
  secondsToWait?: number
  /**
   * A Wallet interface, such as Freighter, that has the methods `isConnected`, `isAllowed`, `getUserInfo`, and `signTransaction`. If not provided, will attempt to import and use Freighter. Example:
   *
   * ```ts
   * import freighter from "@stellar/freighter-api";
   *
   * // later, when calling this function:
   *   wallet: freighter,
   */
  wallet?: Wallet
} = {}) {
    return await invoke({
        method: 'coords',
        args: [((i) => xdr.ScVal.scvU32(i))(token_id)],
        ...options,
        parseResultXdr: (xdr): Option<[u32, u32]> => {
            return scValStrToJs(xdr);
        },
    });
}

export type Admin = {tag: "User", values: void};

function AdminToXdr(admin?: Admin): xdr.ScVal {
    if (!admin) {
        return xdr.ScVal.scvVoid();
    }
    let res: xdr.ScVal[] = [];
    switch (admin.tag) {
        case "User":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("User"));
            break;  
    }
    return xdr.ScVal.scvVec(res);
}

function AdminFromXdr(base64Xdr: string): Admin {
    type Tag = Admin["tag"];
    type Value = Admin["values"];
    let [tag, values] = strToScVal(base64Xdr).vec()!.map(scValToJs) as [Tag, Value];
    if (!tag) {
        throw new Error('Missing enum tag when decoding Admin from XDR');
    }
    return { tag, values } as Admin;
}

export type DataKey = {tag: "Balance", values: [Address]} | {tag: "TokenOwner", values: [u32]} | {tag: "Approved", values: [u32]} | {tag: "Operator", values: [Address, Address]};

function DataKeyToXdr(dataKey?: DataKey): xdr.ScVal {
    if (!dataKey) {
        return xdr.ScVal.scvVoid();
    }
    let res: xdr.ScVal[] = [];
    switch (dataKey.tag) {
        case "Balance":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Balance"));
            res.push(((i)=>addressToScVal(i))(dataKey.values[0]));
            break;
    case "TokenOwner":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("TokenOwner"));
            res.push(((i)=>xdr.ScVal.scvU32(i))(dataKey.values[0]));
            break;
    case "Approved":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Approved"));
            res.push(((i)=>xdr.ScVal.scvU32(i))(dataKey.values[0]));
            break;
    case "Operator":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Operator"));
            res.push(((i)=>addressToScVal(i))(dataKey.values[0]));
            res.push(((i)=>addressToScVal(i))(dataKey.values[1]));
            break;  
    }
    return xdr.ScVal.scvVec(res);
}

function DataKeyFromXdr(base64Xdr: string): DataKey {
    type Tag = DataKey["tag"];
    type Value = DataKey["values"];
    let [tag, values] = strToScVal(base64Xdr).vec()!.map(scValToJs) as [Tag, Value];
    if (!tag) {
        throw new Error('Missing enum tag when decoding DataKey from XDR');
    }
    return { tag, values } as DataKey;
}

export type DatakeyMetadata = {tag: "Name", values: void} | {tag: "Symbol", values: void} | {tag: "Uri", values: [u32]};

function DatakeyMetadataToXdr(datakeyMetadata?: DatakeyMetadata): xdr.ScVal {
    if (!datakeyMetadata) {
        return xdr.ScVal.scvVoid();
    }
    let res: xdr.ScVal[] = [];
    switch (datakeyMetadata.tag) {
        case "Name":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Name"));
            break;
    case "Symbol":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Symbol"));
            break;
    case "Uri":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("Uri"));
            res.push(((i)=>xdr.ScVal.scvU32(i))(datakeyMetadata.values[0]));
            break;  
    }
    return xdr.ScVal.scvVec(res);
}

function DatakeyMetadataFromXdr(base64Xdr: string): DatakeyMetadata {
    type Tag = DatakeyMetadata["tag"];
    type Value = DatakeyMetadata["values"];
    let [tag, values] = strToScVal(base64Xdr).vec()!.map(scValToJs) as [Tag, Value];
    if (!tag) {
        throw new Error('Missing enum tag when decoding DatakeyMetadata from XDR');
    }
    return { tag, values } as DatakeyMetadata;
}

export type DataKeyEnumerable = {tag: "IndexToken", values: void} | {tag: "TokenIndex", values: void} | {tag: "OwnerIndexToken", values: [Address]} | {tag: "OwnerTokenIndex", values: [Address]};

function DataKeyEnumerableToXdr(dataKeyEnumerable?: DataKeyEnumerable): xdr.ScVal {
    if (!dataKeyEnumerable) {
        return xdr.ScVal.scvVoid();
    }
    let res: xdr.ScVal[] = [];
    switch (dataKeyEnumerable.tag) {
        case "IndexToken":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("IndexToken"));
            break;
    case "TokenIndex":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("TokenIndex"));
            break;
    case "OwnerIndexToken":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("OwnerIndexToken"));
            res.push(((i)=>addressToScVal(i))(dataKeyEnumerable.values[0]));
            break;
    case "OwnerTokenIndex":
            res.push(((i) => xdr.ScVal.scvSymbol(i))("OwnerTokenIndex"));
            res.push(((i)=>addressToScVal(i))(dataKeyEnumerable.values[0]));
            break;  
    }
    return xdr.ScVal.scvVec(res);
}

function DataKeyEnumerableFromXdr(base64Xdr: string): DataKeyEnumerable {
    type Tag = DataKeyEnumerable["tag"];
    type Value = DataKeyEnumerable["values"];
    let [tag, values] = strToScVal(base64Xdr).vec()!.map(scValToJs) as [Tag, Value];
    if (!tag) {
        throw new Error('Missing enum tag when decoding DataKeyEnumerable from XDR');
    }
    return { tag, values } as DataKeyEnumerable;
}

const Errors = [ 
{message:""},
  {message:""},
  {message:""},
  {message:""}
]