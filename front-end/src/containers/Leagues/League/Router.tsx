import React, { FunctionComponent } from 'react';
import { Switch, Route } from 'react-router-dom';
import { Tabs } from './Tabs';
import { Forwarder } from './Forwarder';
import { DraftSelection } from './DraftSelection';
import { JoinLeague } from './Join';
import { Lineups } from './Lineups';
import { Swaps } from './Swaps';
import { Scoreboard } from './Scoreboard';

export const LeagueRouter: FunctionComponent = (props) => {
  return (
    <>
      <div style={{ height: '57px' }}></div>
      <Tabs />
      <Switch>
        <Route path="/leagues/:index" exact component={Forwarder} />
        <Route path="/leagues/:index/draft-selection" exact component={DraftSelection} />
        <Route path="/leagues/:index/join" exact component={JoinLeague} />
        <Route path="/leagues/:index/lineups" exact component={Lineups} />
        <Route path="/leagues/:index/swaps" exact component={Swaps} />
        <Route path="/leagues/:index/scoreboard" exact component={Scoreboard} />
      </Switch>
    </>
  );
};
