import React from 'react';
import { MemoryRouter, Switch, Route } from 'react-router-dom';

import Container from 'react-bootstrap/Container';
import { LinkContainer } from 'react-router-bootstrap';

import Nav from 'react-bootstrap/Nav';

import './App.css';
import Home from './pages/home';
import Item from './pages/about';


const Login = () => <span>Login</span>;

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
          <LinkContainer to="/login"><Nav.Link>Login</Nav.Link></LinkContainer>
        </Nav.Item>
      </Nav>
      <h2>
        <Switch>
          <Route path="/about">
            <Item />
            <Item />
            <Item />
          </Route>
          <Route path="/login">
            <Login />
          </Route>
          <Route path="/home">
            <Home />
          </Route>
        </Switch>
      </h2>
    </Container>
  </MemoryRouter>
);

export default App;
