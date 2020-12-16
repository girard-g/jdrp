import { useKeycloak } from '@react-keycloak/web';

const AppJdrp = () => {
    const keycloak = useKeycloak();
    
    return <span>Oui</span>;

    <div>{`User is ${
        !keycloak.keycloak.authenticated ? 'NOT ' : ''
      }authenticated`}</div>


};

export default AppJdrp;