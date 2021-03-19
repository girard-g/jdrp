import React from 'react';
import Popover from 'react-bootstrap/Popover';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Card from 'react-bootstrap/Card';

class Jewel extends React.Component {
    render() {
        let itemspec;
        let border;

        if (this.props.item.object.equipement.jewel.spell.incante !== "") {
            itemspec = <Card.Subtitle className="mb-2 text-muted"><b>Spell damage:</b> {this.props.item.object.equipement.jewel.spell.incante}</Card.Subtitle>
        } else {
            itemspec = ''
        }

        if (this.props.item.object.rarity == "common") {
            border = "item_render_div_common"
        } else if (this.props.item.object.rarity == "magic") {
            border = "item_render_div_magic"
        } else if (this.props.item.object.rarity == "rare") {
            border = "item_render_div_rare"
        } else if (this.props.item.object.rarity == "epic") {
            border = "item_render_div_epic"
        } else {
            border = "item_render_div_legendary"
        }

        const popover = (
            <Popover id="popover-basic">
                <Card style={{ width: '18rem', color: "black" }}>
                    <Card.Body>
                        <Card.Title>{this.props.item.object.name}</Card.Title>
                        <hr></hr>
                        <Card.Subtitle className="mb-2 text-muted"><b>Type:</b> {this.props.item.object.equipement.jewel.jewel_type}</Card.Subtitle>
                        <Card.Subtitle className="mb-2 text-muted"><b>Rarity:</b> {this.props.item.object.rarity}</Card.Subtitle>
                        <Card.Subtitle className="mb-2 text-muted"><b>Emplacement:</b> {this.props.item.object.equipement.slot}</Card.Subtitle>
                        <Card.Subtitle className="mb-2 text-muted"><b>Spell name:</b> {this.props.item.object.equipement.jewel.spell.name}</Card.Subtitle>
                        <Card.Subtitle className="mb-2 text-muted"><b>Incante:</b> {this.props.item.object.equipement.jewel.spell.incante}</Card.Subtitle>
                        <Card.Subtitle className="mb-2 text-muted"><b>Usage:</b> {this.props.item.object.equipement.jewel.usage}</Card.Subtitle>
                        {itemspec}
                        <Card.Text>
                            {this.props.item.object.equipement.jewel.spell.description}
                        </Card.Text>
                    </Card.Body>
                </Card>
            </Popover>
        );

        const asset = 'http://localhost:8000/' + this.props.item.object.asset;
        return (
            <OverlayTrigger trigger="hover" placement="auto" overlay={popover}>
                <div className="item_render_div">
                    <div className={border}>
                        <img className="img" src={asset} />
                    </div>
                </div>
            </OverlayTrigger>

        );
    }
}

export default Jewel