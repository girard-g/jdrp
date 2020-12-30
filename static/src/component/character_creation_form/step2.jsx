import React from 'react';
import Popover from 'react-bootstrap/Popover'
import OverlayTrigger from 'react-bootstrap/OverlayTrigger'
import Badge from 'react-bootstrap/Badge'

const Step2 = (props) => {
    if (props.currentStep !== 2) {
        return null
    }

    const popover = (title, content, mandatory) => (
        <Popover id="popover-basic">
            <Popover.Title as="h3" style={{ color: 'black' }}>{title}</Popover.Title>
            <Popover.Content>
                {content}
                <hr />
                <i>Important pour: {mandatory}</i>
            </Popover.Content>
        </Popover>
    );

    const StrenghPop = () => (

        <OverlayTrigger placement="right" overlay={popover('Force', props.strenghHelper, props.strenghMandatory)}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
    );

    const DexterityPop = () => (

        <OverlayTrigger placement="right" overlay={popover('Dextérité', props.dexterityHelper, props.dexterityMandatory)}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
    );

    const LuckPop = () => (

        <OverlayTrigger placement="right" overlay={popover('Chance', props.luckHelper, props.luckMandatory)}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
    );

    const WillpowerPop = () => (

        <OverlayTrigger placement="right" overlay={popover('Volonté', props.willpowerHelper, props.willpowerMandatory)}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
    );

    const EndurancePop = () => (

        <OverlayTrigger placement="right" overlay={popover('Constitution', props.enduranceHelper, props.enduranceMandatory)}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
    );

    const CharismPop = () => (

        <OverlayTrigger placement="right" overlay={popover('Charisme', props.charismHelper, props.charismMandatory)}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
    );

    const PerceptionPop = () => (

        <OverlayTrigger placement="right" overlay={popover('Perception', props.perceptionHelper, props.perceptionMandatory)}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
    );

    const EducationnPop = () => (

        <OverlayTrigger placement="right" overlay={popover('Education', props.educationHelper, props.educationMandatory)}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
    );
    return (


        <div className="form-group">
            <label htmlFor="strengh">Force</label>
            <div className="float-right">
                <StrenghPop />
            </div>
            <div class="input-group">
                <button className="btn btn-outline-primary" type="button" name="strengh" value="-10" onClick={props.onclickHandler} >-10</button>
                <button className="btn btn-outline-primary" type="button" name="strengh" value="-1" onClick={props.onclickHandler} >-1</button>
                <input
                    className="form-control"
                    id="strengh"
                    name="strengh"
                    type="number"
                    disabled
                    min={props.min_point}
                    max={props.max_point}
                    value={props.strengh}
                />
                <button className="btn btn-outline-primary" type="button" name="strengh" value="+1" onClick={props.onclickHandler} >+1</button>
                <button className="btn btn-outline-primary" type="button" name="strengh" value="+10" onClick={props.onclickHandler}>+10</button>
            </div>


            <label htmlFor="dexterity">Dexterité</label>
            <div className="float-right">
                <DexterityPop />
            </div>
            <div class="input-group">
                <button className="btn btn-outline-primary" type="button" name="dexterity" value="-10" onClick={props.onclickHandler} >-10</button>
                <button className="btn btn-outline-primary" type="button" name="dexterity" value="-1" onClick={props.onclickHandler} >-1</button>
                <input
                    className="form-control"
                    id="dexterity"
                    name="dexterity"
                    type="number"
                    disabled
                    min={props.min_point}
                    max={props.max_point}
                    value={props.dexterity}
                />
                <button className="btn btn-outline-primary" type="button" name="dexterity" value="+1" onClick={props.onclickHandler} >+1</button>
                <button className="btn btn-outline-primary" type="button" name="dexterity" value="+10" onClick={props.onclickHandler}>+10</button>
            </div>


            <label htmlFor="endurance">Constitution</label>
            <div className="float-right">
                <EndurancePop />
            </div>
            <div class="input-group">
                <button className="btn btn-outline-primary" type="button" name="endurance" value="-10" onClick={props.onclickHandler} >-10</button>
                <button className="btn btn-outline-primary" type="button" name="endurance" value="-1" onClick={props.onclickHandler} >-1</button>
                <input
                    className="form-control"
                    id="endurance"
                    name="endurance"
                    type="number"
                    disabled
                    min={props.min_point}
                    max={props.max_point}
                    value={props.endurance}
                />
                <button className="btn btn-outline-primary" type="button" name="endurance" value="+1" onClick={props.onclickHandler} >+1</button>
                <button className="btn btn-outline-primary" type="button" name="endurance" value="+10" onClick={props.onclickHandler}>+10</button>
            </div>


            <label htmlFor="charism">Charisme</label>
            <div className="float-right">
                <CharismPop />
            </div>
            <div class="input-group">
                <button className="btn btn-outline-primary" type="button" name="charism" value="-10" onClick={props.onclickHandler} >-10</button>
                <button className="btn btn-outline-primary" type="button" name="charism" value="-1" onClick={props.onclickHandler} >-1</button>
                <input
                    className="form-control"
                    id="charism"
                    name="charism"
                    type="number"
                    disabled
                    min={props.min_point}
                    max={props.max_point}
                    value={props.charism}
                />
                <button className="btn btn-outline-primary" type="button" name="charism" value="+1" onClick={props.onclickHandler} >+1</button>
                <button className="btn btn-outline-primary" type="button" name="charism" value="+10" onClick={props.onclickHandler}>+10</button>
            </div>


            <label htmlFor="perception">Perception</label>
            <div className="float-right">
                <PerceptionPop />
            </div>
            <div class="input-group">
                <button className="btn btn-outline-primary" type="button" name="perception" value="-10" onClick={props.onclickHandler} >-10</button>
                <button className="btn btn-outline-primary" type="button" name="perception" value="-1" onClick={props.onclickHandler} >-1</button>
                <input
                    className="form-control"
                    id="perception"
                    name="perception"
                    type="number"
                    disabled
                    min={props.min_point}
                    max={props.max_point}
                    value={props.perception}
                />
                <button className="btn btn-outline-primary" type="button" name="perception" value="+1" onClick={props.onclickHandler} >+1</button>
                <button className="btn btn-outline-primary" type="button" name="perception" value="+10" onClick={props.onclickHandler}>+10</button>
            </div>


            <label htmlFor="luck">Chance</label>
            <div className="float-right">
                <LuckPop />
            </div>
            <div class="input-group">
                <button className="btn btn-outline-primary" type="button" name="luck" value="-10" onClick={props.onclickHandler} >-10</button>
                <button className="btn btn-outline-primary" type="button" name="luck" value="-1" onClick={props.onclickHandler} >-1</button>
                <input
                    className="form-control"
                    id="luck"
                    name="luck"
                    type="number"
                    disabled
                    min={props.min_point}
                    max={props.max_point}
                    value={props.luck}
                />
                <button className="btn btn-outline-primary" type="button" name="luck" value="+1" onClick={props.onclickHandler} >+1</button>
                <button className="btn btn-outline-primary" type="button" name="luck" value="+10" onClick={props.onclickHandler}>+10</button>
            </div>

            <label htmlFor="willpower">Volonté</label>
            <div className="float-right">
                <WillpowerPop />
            </div>
            <div class="input-group">
                <button className="btn btn-outline-primary" type="button" name="willpower" value="-10" onClick={props.onclickHandler} >-10</button>
                <button className="btn btn-outline-primary" type="button" name="willpower" value="-1" onClick={props.onclickHandler} >-1</button>
                <input
                    className="form-control"
                    id="willpower"
                    name="willpower"
                    type="number"
                    disabled
                    min={props.min_point}
                    max={props.max_point}
                    value={props.willpower}
                />
                <button className="btn btn-outline-primary" type="button" name="willpower" value="+1" onClick={props.onclickHandler} >+1</button>
                <button className="btn btn-outline-primary" type="button" name="willpower" value="+10" onClick={props.onclickHandler}>+10</button>
            </div>

            <label htmlFor="education">Education</label>
            <div className="float-right">
                <EducationnPop />
            </div>
            <div class="input-group">
                <button className="btn btn-outline-primary" type="button" name="education" value="-10" onClick={props.onclickHandler} >-10</button>
                <button className="btn btn-outline-primary" type="button" name="education" value="-1" onClick={props.onclickHandler} >-1</button>
                <input
                    className="form-control"
                    id="education"
                    name="education"
                    type="number"
                    disabled
                    min={props.min_point}
                    max={props.max_point}
                    value={props.education}
                />
                <button className="btn btn-outline-primary" type="button" name="education" value="+1" onClick={props.onclickHandler} >+1</button>
                <button className="btn btn-outline-primary" type="button" name="education" value="+10" onClick={props.onclickHandler}>+10</button>
            </div>
        </div>
    );
}

export default Step2;