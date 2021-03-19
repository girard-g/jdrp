import React, { useState, useEffect } from 'react';
import { useKeycloak } from '@react-keycloak/web';
import { Spinner } from 'react-bootstrap';
import axios from 'axios';
import CTabs from '../../component/board/CTabs';
import MasterForm from '../../component/character_creation_form/character_creation_form.jsx';

const AppJdrp = () => {

    const keycloak = useKeycloak();

    const [error, setError] = useState(null);
    const [isLoaded, setisLoaded] = useState(false);
    const [rep, setRep] = useState(null);

    useEffect(() => {
        axios.post("http://localhost:8000/api/getcharacter", {
            withCredentials: true,
            credentials: 'include',
        }, {
            headers: { 'Authorization': keycloak.keycloak.token }
        })
            .then(
                response => (
                    setRep(response.data)
                ),
                (error) => {
                    setError(error)
                }
            )
            .then(
                () => setisLoaded(true)
            )
    }, []);

    if (!isLoaded) {
        return <Spinner animation="border" role="status" variant="primary"><span className="sr-only">Loading...</span></Spinner>;
    }

    if (error) {
        return <div>Erreur : {error.message}</div>;
    }

    return (
        <>
            {rep === 'FALSE' &&
                (<MasterForm k={keycloak} />) ||
                (<CTabs c={JSON.parse(rep)} />)
                // (<>{console.log(rep)}</>)
            }
        </>
    )
};

export default AppJdrp;