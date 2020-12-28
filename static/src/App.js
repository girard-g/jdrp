import React from 'react';
import { BrowserRouter, Switch, Route } from 'react-router-dom';

import {ReactKeycloakProvider} from '@react-keycloak/web';
import keycloak from './keycloak';

import './App.css';
import Home from './pages/home';
import About from './pages/about';
import AppJdrp from "./pages/private/appjdrp";
import Navigation from './component/nav';
import PrivateRoute from './utilities/privatesRoutes';

const App = () => (

  <ReactKeycloakProvider authClient={keycloak}>
    <BrowserRouter>
      <Navigation />
      <Switch>
        {/* <Route path="/home" component={() => <Home keycloak={keycloak} />} /> */}
        <Route path="/home" component={Home} />
        <Route path="/about" component={About} />
        <PrivateRoute path="/app" component={AppJdrp} />
      </Switch>
    </BrowserRouter>
  </ReactKeycloakProvider>


);



export default App;

