(function(){function r(e,n,t){function o(i,f){if(!n[i]){if(!e[i]){var c="function"==typeof require&&require;if(!f&&c)return c(i,!0);if(u)return u(i,!0);var a=new Error("Cannot find module '"+i+"'");throw a.code="MODULE_NOT_FOUND",a}var p=n[i]={exports:{}};e[i][0].call(p.exports,function(r){var n=e[i][1][r];return o(n||r)},p,p.exports,r,e,n,t)}return n[i].exports}for(var u="function"==typeof require&&require,i=0;i<t.length;i++)o(t[i]);return o}return r})()({1:[function(require,module,exports){
module.exports = function() {

    const stats = {
        "race": {
            "troll":{
                "strengh": 15,
                "education":-5,
                "charism":-10,
            },
            "elf":{
                "willpower": 10,
                "charism":5,
                "force":-10,
            },
            "humain":{
                "dexterity": 10,
                "endurance":-10,
            },
            "demiElf":{
                "willpower": 5,
                "education": 5,
                "endurance":-5,
                "force":-5,
            },
            "nain":{
                "endurance": 5,
                "force": 5,
                "dexterity":-10,
                "charism":-5,
            },
            "saurial":{
                "dexterity": 5,
                "perception": 5,
                "charism":-5,
                "education":-5,
                "endurance":-5,
            },
        },
        "class":{
            "guerrier":{
                "force": 10,
                "endurance":10,
                "willpower":-20,
            },
            "mage":{
                "willpower": 20,
                "force":-20,
                "education":10,
            },
            "voleur":{
                "dexterity": 20,
                "force":-15,
                "endurance":-15,
            },
            "archer":{
                "dexterity": 10,
                "endurance":-10,
                "education":5,
                "perception":5,
            },
            "monk":{
                "force": 5,
                "dexterity":5,
                "endurance":5,
                "education":5,
                "charism":-15,
            },
            "drood":{
                "force": 10,
                "willpower":5,
                "charism":-5,
                "education":-5,
            },
            "paladin":{
                "endurance": 10,
                "willpower":5,
                "dexterity":-10,
                "force":-5,
            },
            "alchimist":{
                "force": 15,
                "education":15,
                "willpower":-10,
                "charism":15,
                "endurance":-5,
            },
        }
    }

    return stats;
  }
},{}],2:[function(require,module,exports){
var caracterStats = require('./caracters_stats.js');


var HttpClient = function() {
    this.get = function(aUrl, aCallback) {
        let anHttpRequest = new XMLHttpRequest();
        anHttpRequest.onreadystatechange = function() {
            if (anHttpRequest.readyState === 4 && anHttpRequest.status === 200)
                aCallback(anHttpRequest.responseText);
        };

        anHttpRequest.open( "GET", aUrl, true );
        anHttpRequest.send( null );
    }
};

function getCookie(cname) {
    let name = cname + "=";
    let decodedCookie = decodeURIComponent(document.cookie);
    let ca = decodedCookie.split(';');
    for(let i = 0; i <ca.length; i++) {
        let c = ca[i];
      while (c.charAt(0) == ' ') {
        c = c.substring(1);
      }
      if (c.indexOf(name) == 0) {
        return c.substring(name.length, c.length);
      }
    }
    return "";
  }

let client = new HttpClient();
let user_id = getCookie("user_id");

// if(user_id !== "") {
//     client.get('http://localhost:8000/player-stats/'+user_id, function(response) {
    
//     })
// }

let toto = caracterStats();

document.getElementById('raceSelect').addEventListener('change', function(e) {
    let totot = document.getElementById('tip');
    let txt  = "";

    if(this.value == 1){
        txt = "<strong>Race attributs:</strong> "+toto.race.troll.strengh+" force  "+toto.race.troll.education+" education "+toto.race.troll.charism+" charisme";
    }else if (this.value == 2){
        txt = "<strong>Race attributs:</strong> "+toto.race.elf.willpower+" volonté  "+toto.race.elf.charism+" charisme  "+toto.race.elf.force+" force";
    }else if (this.value == 3){
        txt = "<strong>Race attributs:</strong> "+toto.race.humain.dexterity+" dextérité "+toto.race.humain.endurance+" constitution";
    }else if (this.value == 4){
        txt = "<strong>Race attributs:</strong> "+toto.race.demiElf.willpower+" volonté "+toto.race.demiElf.education+" éducation "+toto.race.demiElf.endurance+" constitution "+toto.race.demiElf.force+" force";
    }else if (this.value == 5){
        txt = "<strong>Race attributs:</strong> "+toto.race.nain.endurance+" constitution  "+toto.race.nain.force+" force  "+toto.race.nain.dexterity+" dextérité  "+toto.race.nain.charism+" charisme";
    }else if (this.value == 6){
        txt = "<strong>Race attributs:</strong> "+toto.race.saurial.dexterity+" dextérité "+toto.race.saurial.perception+" perception "+toto.race.saurial.charism+" charisme, "+toto.race.saurial.education+" éducation "+toto.race.saurial.endurance+" constitution";
    }

    totot.innerHTML = txt;
});


document.getElementById('classSelect').addEventListener('change', function(e) {
    var totot = document.getElementById('top');
    let txt  = "";
    if(this.value == 1){
        txt = "<strong>Class attributs:</strong> "+toto.class.guerrier.force+" force "+toto.class.guerrier.endurance+" constitution "+toto.class.guerrier.willpower+" volonté, peut maitriser toutes les armes";
    }else if (this.value == 2){
        txt = "<strong>Class attributs:</strong> "+toto.class.mage.willpower+" volonté "+toto.class.mage.force+" en force "+toto.class.mage.education+" en éducation (3 sorts au choix)";
    }else if (this.value == 3){
        txt = "<strong>Class attributs:</strong> "+toto.class.voleur.dexterity+" en dextérité "+toto.class.voleur.force+" en force "+toto.class.voleur.endurance+" constitution ( spécialiste des attaques sournoises, à des aptitudes d’attaques dans le dos)";
    }else if (this.value == 4){
        txt = "<strong>Class attributs:</strong> "+toto.class.archer.dexterity+" dextérité "+toto.class.archer.endurance+" en constitution "+toto.class.archer.education+" en éducation "+toto.class.archer.perception+" perception (Utilise des arcs, est un pisteur et possède un familier possédant le quart de ses aptitudes)";
    }else if (this.value == 5){
        txt = "<strong>Class attributs:</strong> "+toto.class.monk.force+" en force "+toto.class.monk.dexterity+" dextérité "+toto.class.monk.endurance+" constitution "+toto.class.monk.education+" éducation "+toto.class.monk.charism+" charisme (Bercé dans les mantras, le moine peut soigner ses alliés (1d10) c’est aussi un expert du combat à mains nues. Ne porte pas d’armes).";
    }else if (this.value == 6){
        txt = "<strong>Class attributs:</strong> "+toto.class.drood.force+" force, "+toto.class.drood.force+" volonté "+toto.class.drood.charism+" charisme "+toto.class.drood.education+" éducation (Classe de nature, peut dompter les animaux. Possède une capacité : Forme animal. 0 définir au dés. Peut se changer une fois par jour)";
    }else if (this.value == 7){
        txt = "<strong>Class attributs:</strong> "+toto.class.paladin.endurance+" constitution, "+toto.class.paladin.willpower+" volonté, "+toto.class.paladin.dexterity+" dextérité, "+toto.class.paladin.force+" force (apposition des mains, heal de 1d10).";
    }else if (this.value == 8){
        txt = "<strong>Class attributs:</strong> "+toto.class.alchimist.force+" force, "+toto.class.alchimist.education+" education, "+toto.class.alchimist.willpower+" volonté, "+toto.class.alchimist.charism+" charisme, "+toto.class.alchimist.endurance+" constitution (Spécialiste des herbes et cataplasmes, peu crafter des remèdes, poisons etc …)";
    }

    totot.innerHTML = txt;
});


document.getElementById('check').addEventListener('click', function(e){

    e.preventDefault();

    let raceID = document.getElementById("raceSelect").options[document.getElementById("raceSelect").selectedIndex].value;
    let classID = document.getElementById("classSelect").options[document.getElementById("classSelect").selectedIndex].value;

    let force  = document.getElementById("Force")
    let dexterity = document.getElementById("Dexterite")
    let luck = document.getElementById("Chance")
    let willpower = document.getElementById("Volonte")
    let endurance = document.getElementById("Constitution")
    let charism = document.getElementById("Charisme")
    let perception = document.getElementById("Perception")
    let education = document.getElementById("Education")

    force.value = 0;
    dexterity.value = 0;
    luck.value = 0;
    willpower.value = 0;
    endurance.value = 0;
    charism.value = 0;
    perception.value = 0;
    education.value = 0;

    if(raceID == 1){
        
        if(force.value == 0 || force.value == null ){
            force.value = toto.race.troll.strengh
        }else if(force.value == toto.race.troll.strengh){
            force.value = force.value
        }

        if(education.value == 0 || education.value == null){
            education.value = toto.race.troll.education
        }else if(education.value == toto.race.troll.education){
            education.value = education.value
        }

        if(charism.value == 0 || charism.value == null){
            charism.value = toto.race.troll.charism
        }else if(charism.value == toto.race.troll.charism){
            charism.value = charism.value
        }
       
    }else if (raceID == 2){

        if(force.value == 0){
            force.value = toto.race.elf.force
        }else if(force.value == toto.race.elf.force){
            force.value = force.value
        }

        if(willpower.value == 0){
            willpower.value = toto.race.elf.willpower
        }else if(willpower.value == toto.race.elf.willpower){
            willpower.value = willpower.value
        }

        if(charism.value == 0){
            charism.value = toto.race.elf.charism
        }else if(education.value == toto.race.elf.charism){
            charism.value = charism.value
        }

    }else if (raceID == 3){

        if(dexterity.value == 0){
            dexterity.value =  toto.race.humain.dexterity
        }else if(dexterity.value == toto.race.humain.dexterity){
            dexterity.value = dexterity.value
        }

        if(endurance.value == 0){
            endurance.value = toto.race.humain.endurance
        }else if(endurance.value ==  toto.race.humain.endurance){
            endurance.value = endurance.value
        }

    }else if (raceID == 4){

        if(willpower.value == 0){
            willpower.value = toto.race.demiElf.willpower
        }else if(willpower.value == toto.race.demiElf.willpower){
            willpower.value = willpower.value
        }

        if(endurance.value == 0){
            endurance.value =  toto.race.demiElf.endurance
        }else if(endurance.value == toto.race.demiElf.endurance){
            endurance.value = endurance.value
        }

        if(education.value == 0){
            education.value = toto.race.demiElf.education
        }else if(education.value == toto.race.demiElf.education){
            education.value = education.value
        }

        if(force.value == 0){
            force.value = toto.race.demiElf.force
        }else if(force.value ==  toto.race.demiElf.force){
            force.value = force.value
        }

    }else if (raceID == 5){

        if(dexterity.value == 0){
            dexterity.value = toto.race.nain.dexterity
        }else if(dexterity.value == toto.race.nain.dexterity){
            dexterity.value = dexterity.value
        }

        if(endurance.value == 0){
            endurance.value = toto.race.nain.endurance
        }else if(endurance.value == toto.race.nain.endurance){
            endurance.value = endurance.value
        }

        if(charism.value == 0){
            charism.value = toto.race.nain.charism
        }else if(charism.value == toto.race.nain.charism){
            charism.value = charism.value
        }

        if(force.value == 0){
            force.value = toto.race.nain.force
        }else if(force.value == toto.race.nain.force){
            force.value = force.value
        }

    }else if (raceID == 6){

        if(dexterity.value == 0){
            dexterity.value = toto.race.saurial.dexterity
        }else if(dexterity.value == toto.race.saurial.dexterity){
            dexterity.value = dexterity.value
        }

        if(endurance.value == 0){
            endurance.value =  toto.race.saurial.endurance
        }else if(endurance.value ==  toto.race.saurial.endurance){
            endurance.value = endurance.value
        }

        if(charism.value == 0){
            charism.value =  toto.race.saurial.charism
        }else if(charism.value ==  toto.race.saurial.charism){
            charism.value = charism.value
        }

        if(perception.value == 0){
            perception.value =  toto.race.saurial.perception
        }else if(perception.value ==  toto.race.saurial.perception){
            perception.value = perception.value
        }

        if(education.value == 0){
            education.value =  toto.race.saurial.education
        }else if(education.value ==  toto.race.saurial.education){
            education.value = education.value
        }
    }

    if(classID == 1){

        if(force.value == 0 || force.value == null){
            force.value = toto.class.guerrier.force
        }else if(force.value == toto.class.guerrier.force){
            force.value = force.value
        }else{
            force.value =  parseInt(force.value) + toto.class.guerrier.force;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = toto.class.guerrier.endurance
        }else if(endurance.value == toto.class.guerrier.endurance){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) + toto.class.guerrier.endurance;
        }

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = toto.class.guerrier.willpower
        }else if(willpower.value == toto.class.guerrier.willpower){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) + toto.class.guerrier.willpower;
        }

    }else if (classID == 2){

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = toto.class.mage.willpower
        }else if(willpower.value == toto.class.mage.willpower){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) + toto.class.mage.willpower;
        }

        if(force.value == 0 || force.value == null){
            force.value = toto.class.mage.force
        }else if(force.value == toto.class.mage.force){
            force.value = force.value
        }else{
            force.value =  parseInt(force.value) + toto.class.mage.force;
        }

        if(education.value == 0 || education.value == null){
            education.value = toto.class.mage.education
        }else if(education.value == toto.class.mage.education){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) + toto.class.mage.education;
        }

    }else if (classID == 3){

        if(dexterity.value == 0 || dexterity.value == null){
            dexterity.value = toto.class.voleur.dexterity
        }else if(dexterity.value == toto.class.voleur.dexterity){
            dexterity.value = dexterity.value
        }else{
            dexterity.value = parseInt(dexterity.value) + toto.class.voleur.dexterity;
        }

        if(force.value == 0 || force.value == null){
            force.value = toto.class.voleur.force
        }else if(force.value == toto.class.voleur.force){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) + toto.class.voleur.force;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = toto.class.voleur.endurance
        }else if(endurance.value == toto.class.voleur.endurance){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) + toto.class.voleur.endurance;
        }

    }else if (classID == 4){

        if(dexterity.value == 0 || dexterity.value == null){
            dexterity.value = toto.class.archer.dexterity
        }else if(dexterity.value == toto.class.archer.dexterity){
            dexterity.value = dexterity.value
        }else{
            dexterity.value = parseInt(dexterity.value) + toto.class.archer.dexterity;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = toto.class.archer.endurance
        }else if(endurance.value == toto.class.archer.endurance){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) + toto.class.archer.endurance;
        }

        if(education.value == 0 || education.value == null){
            education.value = toto.class.archer.education
        }else if(education.value == toto.class.archer.education){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) + toto.class.archer.education;
        }

        if(perception.value == 0 || perception.value == null){
            perception.value = toto.class.archer.perception
        }else if(perception.value == toto.class.archer.perception){
            perception.value = perception.value
        }else{
            perception.value = parseInt(perception.value) + toto.class.archer.perception;
        }

    }else if (classID == 5){

        if(force.value == 0 || force.value == null){
            force.value = toto.class.monk.force
        }else if(force.value == toto.class.monk.force ){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) + toto.class.monk.force;
        }

        if(dexterity.value == 0 || dexterity.value == null){
            dexterity.value = toto.class.monk.dexterity
        }else if(dexterity.value == toto.class.monk.dexterity){
            dexterity.value = dexterity.value
        }else{
            dexterity.value = parseInt(dexterity.value) + toto.class.monk.dexterity;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = toto.class.monk.endurance
        }else if(endurance.value == toto.class.monk.endurance){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) + toto.class.monk.endurance;
        }

        if(education.value == 0 || education.value == null){
            education.value = toto.class.monk.education
        }else if(education.value == toto.class.monk.education){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) + toto.class.monk.education;
        }

        if(charism.value == 0 || charism.value == null){
            charism.value = toto.class.monk.charism
        }else if(charism.value == toto.class.monk.charism){
            charism.value = charism.value
        }else{
            charism.value = parseInt(charism.value) + toto.class.monk.charism;
        }

    }else if (classID == 6){

        if(force.value == 0 || force.value == null){
            force.value = toto.class.drood.force
        }else if(force.value == toto.class.drood.force){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) + toto.class.drood.force;
        }

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = toto.class.drood.willpower
        }else if(willpower.value == toto.class.drood.willpower){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) + toto.class.drood.willpower;
        }

        if(charism.value == 0 || charism.value == null){
            charism.value = toto.class.drood.charism
        }else if(charism.value == toto.class.drood.charism){
            charism.value = charism.value
        }else{
            charism.value = parseInt(charism.value) + toto.class.drood.charism;
        }

        if(education.value == 0 || education.value == null){
            education.value = toto.class.drood.education
        }else if(education.value == toto.class.drood.education){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) + toto.class.drood.education;
        }

    }
    else if (classID == 7){
        if(endurance.value == 0 || endurance.value == null){
            endurance.value = toto.class.paladin.endurance
        }else if(endurance.value == toto.class.paladin.endurance){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) + toto.class.paladin.endurance;
        }

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = toto.class.paladin.willpower
        }else if(willpower.value == toto.class.paladin.willpower){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) + toto.class.paladin.willpower;
        }

        if(dexterity.value == 0 || dexterity.value == null){
            dexterity.value = toto.class.paladin.dexterity
        }else if(dexterity.value == toto.class.paladin.dexterity){
            dexterity.value = dexterity.value
        }else{
            dexterity.value = parseInt(dexterity.value) + toto.class.paladin.dexterity;
        }

        if(force.value == 0 || force.value == null){
            force.value = toto.class.paladin.force
        }else if(force.value == toto.class.paladin.force){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) + toto.class.paladin.force;
        }
    }
    else if (classID == 8){

        if(force.value == 0 || force.value == null){
            force.value = toto.class.alchimist.force
        }else if(force.value == toto.class.alchimist.force){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) + toto.class.alchimist.force;
        }

        if(education.value == 0 || education.value == null){
            education.value = toto.class.alchimist.education
        }else if(education.value == toto.class.alchimist.education){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) + toto.class.alchimist.education;
        }

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = toto.class.alchimist.willpower
        }else if(willpower.value == toto.class.alchimist.willpower){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) + toto.class.alchimist.willpower;
        }

        if(charism.value == 0 || charism.value == null){
            charism.value = toto.class.alchimist.charism
        }else if(charism.value == toto.class.alchimist.charism){
            charism.value = charism.value
        }else{
            charism.value = parseInt(charism.value) + toto.class.alchimist.charism;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = toto.class.alchimist.endurance
        }else if(endurance.value == toto.class.alchimist.endurance){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) + toto.class.alchimist.endurance;
        }
    }


});

},{"./caracters_stats.js":1}]},{},[2]);