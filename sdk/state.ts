import { BufferLayout } from './util/layout';
import { PublicKey } from '@solana/web3.js';

import * as Layout from './util/layout';

export const MAX_PLAYERS_PER_INSTRUCTION = 255;
export const PLAYERS_CAPACITY = 1000;
export const GAMES_COUNT = 17;
export const LEAGUES_COUNT = 100;
export const LEAGUE_USERS_COUNT = 10;
export const ACTIVE_PLAYERS_COUNT = 8;
export const BENCH_PLAYERS_COUNT = 8;
export const TEAM_PLAYERS_COUNT = ACTIVE_PLAYERS_COUNT + BENCH_PLAYERS_COUNT;
export const LINEUP_LEN = 2 * ACTIVE_PLAYERS_COUNT;
export const SWAP_PROPOSALS_CAPACITY = 20;

export const PUB_KEY_LEN = 32;

export enum Position {
  Uninitialized,
  RB,
  LB,
  DL,
  TE,
  DB,
  QB,
  WR,
  OL,
}

export type UserState = {
  pubKey: PublicKey;
  bench: number[];
  lineups: number[][];
  isInitialized: boolean;
};

export const UserStateLayout: typeof BufferLayout.Structure = BufferLayout.struct([
  Layout.publicKey('pubKey'),
  BufferLayout.seq(BufferLayout.u16(), TEAM_PLAYERS_COUNT, 'bench'),
  BufferLayout.seq(
    BufferLayout.seq(BufferLayout.u16(), ACTIVE_PLAYERS_COUNT),
    GAMES_COUNT,
    'lineups'
  ),
  BufferLayout.u8('isInitialized'),
]);

export type League = {
  bid: number;
  userStates: UserState[];
  isInitialized: boolean;
};

export const LeagueLayout: typeof BufferLayout.Structure = BufferLayout.struct([
  BufferLayout.u8('id'),
  BufferLayout.seq(UserStateLayout, LEAGUE_USERS_COUNT, 'userStates'),
  BufferLayout.u8('isInitialized'),
]);

export type Score = {
  score1: number;
  isInitialized: boolean;
};

export const ScoreLayout: typeof BufferLayout.Structure = BufferLayout.struct([
  BufferLayout.u8('score1'),
  BufferLayout.u8('isInitialized'),
]);


export type SwapProposal = {
  givePlayerId: number;
  wantPlayerId: number;
  isInitialized: boolean;
};

export const SwapProposalLayout: typeof BufferLayout.Structure = BufferLayout.struct([
  BufferLayout.u16('givePlayerId'),
  BufferLayout.u16('wantPlayerId'),
  BufferLayout.u8('isInitialized'),
]);

export type Player = {
  id: number;
  position: Position;
  scores: Score[];
  swapProposals: SwapProposal[];
  isInitialized: boolean;
};

export const PlayerLayout: typeof BufferLayout.Structure = BufferLayout.struct([
  BufferLayout.u16('id'),
  BufferLayout.u8('position'),
  BufferLayout.u8('swapProposalsCount'),
  BufferLayout.seq(ScoreLayout, GAMES_COUNT, 'scores'),
  BufferLayout.seq(SwapProposalLayout, SWAP_PROPOSALS_CAPACITY, 'swapProposals'),
  BufferLayout.u8('isInitialized'),
]);

export type Root = {
  /// Oracle authority used to supply game scores.
  oracleAuthority: PublicKey;
  /// An address of an account that stores the latest state.
  players: Player[];
  /// Leagues
  leagues: League[];
  /// Is `true` if this structure has been initialized
  isInitialized: boolean;
};

export const RootLayout: typeof BufferLayout.Structure = BufferLayout.struct([
  Layout.publicKey('oracleAuthority'),
  BufferLayout.u16('playersCount'),
  BufferLayout.seq(PlayerLayout, PLAYERS_CAPACITY, 'players'),
  BufferLayout.seq(LeagueLayout, LEAGUES_COUNT, 'lagues'),
  BufferLayout.u8('isInitialized'),
]);
