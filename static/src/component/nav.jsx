import React from 'react';
import { NavLink } from 'react-router-dom'
import { Navbar, Nav, Button, DropdownButton, Dropdown } from 'react-bootstrap'
import { useKeycloak } from '@react-keycloak/web';

const Navigation = () => {

    const keycloak = useKeycloak();

    return (
        <Navbar bg="light" expand="lg">
            <Navbar.Brand to="/home">JDRP</Navbar.Brand>
            <Navbar.Toggle aria-controls="basic-navbar-nav" />
            <Navbar.Collapse id="basic-navbar-nav">
                <Nav className="mr-auto">
                    <NavLink className="nav-link" to="/home">Home </NavLink>
                    <NavLink className="nav-link" to="/about">DEBUG </NavLink>
                    <NavLink className="nav-link" to="/app">App </NavLink>
                </Nav>
                {keycloak.keycloak && !keycloak.keycloak.authenticated &&
                    <Button className="float-right" variant="outline-primary" onClick={() => keycloak.keycloak.login()}>Login / Sign up</Button>
                }
                {keycloak.keycloak && keycloak.keycloak.authenticated &&
                    <DropdownButton className="float-right" variant="outline-primary" title={keycloak.keycloak.tokenParsed.preferred_username} id="bg-nested-dropdown" menuAlign="right">
                        <Dropdown.Item eventKey="1" onClick={() => keycloak.keycloak.accountManagement()} >Account</Dropdown.Item>
                        <Dropdown.Item eventKey="2" onClick={() => keycloak.keycloak.logout()} >Logout</Dropdown.Item>
                    </DropdownButton>
                }
            </Navbar.Collapse>
        </Navbar>
    );
}

export default Navigation;