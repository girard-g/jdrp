import React from 'react';
import Popover from 'react-bootstrap/Popover';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Card from 'react-bootstrap/Card';

class Consumable extends React.Component {
    render() {

        const popover = (
            <Popover id="popover-basic">
                <Card style={{ width: '18rem', color: "black" }}>
                    <Card.Body>
                        <Card.Title>{this.props.item.consumable.name}</Card.Title>
                        <hr />
                        <Card.Subtitle className="mb-2 text-muted"><b>Type:</b> {this.props.item.consumable.item_type}</Card.Subtitle>
                        <Card.Subtitle className="mb-2 text-muted"><b>Rarity:</b> {this.props.item.consumable.rarity}</Card.Subtitle>
                        <Card.Text>
                            {this.props.item.consumable.description}
                        </Card.Text>
                    </Card.Body>
                </Card>
            </Popover>
        );

        const asset = 'http://localhost:8000/' + this.props.item.consumable.asset;
        return (
            <OverlayTrigger trigger="hover" placement="auto" overlay={popover}>
                <div className="item_render_div">
                    <div className="item_render_div_common">
                        <img className="img" src={asset} />
                    </div>
                </div>
            </OverlayTrigger>

        );
    }
}

export default Consumable