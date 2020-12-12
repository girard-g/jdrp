import React from 'react';
import Spinner from 'react-bootstrap/Spinner';
import Armor from './items/armor';
import Weapon from './items/weapon';
import Jewel from './items/jewel';
import Consumable from './items/consumable';

class Item extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      error: null,
      isLoaded: false,
      item: null
    };
  }
  componentDidMount() {
    fetch("http://localhost:8000/api/testobjectgenerationlol")
      .then(res => res.json())
      .then(
        (result) => {
          this.setState({
            isLoaded: true,
            item: result
          });
        },
        (error) => {
          this.setState({
            isLoaded: true,
            error
          });
        }
      )
  }

  render() {
    const { error, isLoaded, item } = this.state;
    if (error) {
      return <div>Erreur : {error.message}</div>;
    } else if (!isLoaded) {
      return <Spinner animation="border" role="status"><span className="sr-only">Loading...</span></Spinner>;
    } else {

      if (item.object !== null) {
        if (item.object.equipement.armor !== null) {
          return <Armor item={item} />
        } else if (item.object.equipement.weapon !== null) {
          return <Weapon item={item} />
        } else {
          return <Jewel item={item} />
        }
      } else {
        return <Consumable item={item} />
      }
    }
  }
}

export default Item