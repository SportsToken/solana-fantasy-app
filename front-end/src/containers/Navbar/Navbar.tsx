import React, { useState } from 'react';
import { NavLink, Link } from 'react-router-dom';

export function Navbar() {
  const [, forceRerender] = useState({});

  const walletChangeHook = () => {
    forceRerender({});
  };

  if (!window.walletStatusChangeHooks) {
    window.walletStatusChangeHooks = { navbar: walletChangeHook, walletPage: () => {} };
  } else {
    window.walletStatusChangeHooks.navbar = walletChangeHook;
  }

  return (
    <header id="header" className="fixed-top" style={{ backgroundColor: '#000' }}>
      <div className="container d-flex align-items-center">
        <h1 className="logo mr-auto">
          <Link to="/">AE</Link>
        </h1>
        <nav className="nav-menu d-none d-lg-block">
          <ul>
            <NavElement to="/" label="Home" />
            <NavElement to="/leagues/create" label="Create a League" />
            <NavElement to="/leagues" label="Join a League" />
            {/* <NavElement to="/h2h" label="H2H Matchups" /> */}
            {window.wallet &&
            window.wallet.publicKey === '9AmX84PQg4PoyLwPCHbBy2mSRsjKo1CSJXVXXXfSWZTH' ? (
              <NavElement to="/admin" label="Admin Panel" />
            ) : null}
            {window.wallet === undefined ? (
              <NavElement to="/wallet/import" label="Create an Account" />
            ) : (
              <NavElement
                to="/wallet"
                label={`Welcome, ${window.firstName ? window.firstName : 'New Player!'}`}
              />
            )}
          </ul>
        </nav>
      </div>
    </header>
  );
}

function NavElement(props: { to: string; label: string }) {
  return (
    <NavLink to={props.to} activeClassName="active" exact>
      <label>{props.label}</label>
    </NavLink>
  );
}
