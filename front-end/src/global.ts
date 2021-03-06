import { Connection, PublicKey, Account } from '@solana/web3.js';
import { ClojuredWallet } from './clojured-wallet';
import { SFS } from './sdk/sfs';
import { Buffer as Buffer_ } from 'buffer';
import { Root } from './sdk/state';

declare global {
  interface Window {
    Buffer: typeof Buffer_;
    wallet: ClojuredWallet | undefined;
    walletStatusChangeHooks: { navbar: Function; walletPage: Function };
    leagueTabHook: Function;
    sfsProgramId: PublicKey;
    sfsRoot: PublicKey;
    connection: Connection;
    firstName: string;
    lastName: string;

    sfsSDK: () => Promise<SFS>;
    getCachedRootInfo: (forceUpdate?: boolean) => Promise<Root>;
    getCachedPlayers: () => Promise<
      {
        PlayerID: number; // 19801,
        Name: string; // 'Josh Allen',
        Position: string; // 'QB',
        AverageDraftPosition: number; // 108.9,
      }[]
    >;
  }
}

export {};
