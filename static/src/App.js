import React from 'react';
import { MemoryRouter, Switch, Route } from 'react-router-dom';

import Container from 'react-bootstrap/Container';
import { LinkContainer } from 'react-router-bootstrap';

import Nav from 'react-bootstrap/Nav';

import Keycloak from 'keycloak-js';
import './App.css';
import Home from './pages/home';
import Item from './pages/about';
import AppJdrp from "./pages/appjdrp";

let initOptions = {
    url: 'http://127.0.0.1:8080/auth', realm: 'JDRP', clientId: 'JDRPClient', onLoad: 'login-required'
}

let keycloak = Keycloak(initOptions);

keycloak.init({ onLoad: initOptions.onLoad }).then((auth) => {
  if (!auth) {
  window.location.reload();
  } else {
  console.log("Authenticated");
  
  }
})

const App = () => (
  <MemoryRouter>
    <Container className="p-3">

      <Nav
        variant="pills"
        defaultActiveKey="/home"
      >
        <Nav.Item>
          <LinkContainer to="/home"><Nav.Link>Home</Nav.Link></LinkContainer>
        </Nav.Item>
        <Nav.Item>
          <LinkContainer to="/about"><Nav.Link>About</Nav.Link></LinkContainer>
        </Nav.Item>
        <Nav.Item>
          <LinkContainer to="/app"><Nav.Link>App</Nav.Link></LinkContainer>
        </Nav.Item>
      </Nav>
      <h2>
        <Switch>
          <Route path="/about">
            <Item />
            <Item />
            <Item />
          </Route>
          <Route path="/app">
            <AppJdrp />
          </Route>
          <Route path="/home">
            <Home keycloak={keycloak}/>
          </Route>
        </Switch>
      </h2>
    </Container>
  </MemoryRouter>
);



export default App;

