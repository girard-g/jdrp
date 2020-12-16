import { useKeycloak } from '@react-keycloak/web';
import React from 'react';
import {Redirect, Route} from 'react-router-dom';

const PrivateRoute = ({ component: Component, ...rest}) => {

    const keycloak = useKeycloak();

    const isAuth = () => {
        if(keycloak.keycloak.authenticated){
            return true
        }else{
            return false
        }
    };

    return (
        <Route {...rest} render={props => {
            return isAuth() ? <Component {...props}/> : <Redirect to={{pathname: '/home',}} />
        }}
        />
    )
  }

  export default PrivateRoute;