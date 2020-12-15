import React from 'react';
import { BrowserRouter, Switch, Route } from 'react-router-dom';

import Keycloak from 'keycloak-js';
import './App.css';
import Home from './pages/home';
import About from './pages/about';
import AppJdrp from "./pages/appjdrp";
import Navigation from './component/nav';

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

  <BrowserRouter>
    <Navigation />
    <Switch>
      <Route path="/home" component={() => <Home keycloak={keycloak} />} />
      <Route path="/about" component={About} />
      <Route path="/app" component={AppJdrp} />
    </Switch>
  </BrowserRouter>
);



export default App;

