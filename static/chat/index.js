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

document.getElementById('raceSelect').addEventListener('change', function(e) {
    let totot = document.getElementById('tip');
    let txt  = "";

    if(this.value == 1){
        txt = "<strong>Race attributs:</strong> +15 force -5 education -10 charisme";
    }else if (this.value == 2){
        txt = "<strong>Race attributs:</strong> +10 volonté +5 charisme -10 force";
    }else if (this.value == 3){
        txt = "<strong>Race attributs:</strong> +10 dextérité -10 constitution";
    }else if (this.value == 4){
        txt = "<strong>Race attributs:</strong> +5 volonté +5 éducation -5 constitution -5 force";
    }else if (this.value == 5){
        txt = "<strong>Race attributs:</strong> +5 constitution +5 force -10 dextérité -5 charisme";
    }else if (this.value == 6){
        txt = "<strong>Race attributs:</strong> +5 dextérité +5 perception -5 charisme, -5 éducation -5 constitution";
    }

    totot.innerHTML = txt;
});


document.getElementById('classSelect').addEventListener('change', function(e) {
    var totot = document.getElementById('top');
    let txt  = "";
    if(this.value == 1){
        txt = "<strong>Class attributs:</strong> +10 force +10 constitution -20 volonté, peut maitriser toutes les armes";
    }else if (this.value == 2){
        txt = "<strong>Class attributs:</strong> +20 volonté -20 en force +10 en éducation (3 sorts au choix)";
    }else if (this.value == 3){
        txt = "<strong>Class attributs:</strong> +20 en dextérité -15 en force et constitution ( spécialiste des attaques sournoises, à des aptitudes d’attaques dans le dos)";
    }else if (this.value == 4){
        txt = "<strong>Class attributs:</strong> +10 dextérité -10 en constitution +5 en éducation +5 perception (Utilise des arcs, est un pisteur et possède un familier possédant le quart de ses aptitudes)";
    }else if (this.value == 5){
        txt = "<strong>Class attributs:</strong> +5 en force +5 dextérité +5 constitution +5 éducation -15 charisme (Bercé dans les mantras, le moine peut soigner ses alliés (1d10) c’est aussi un expert du combat à mains nues. Ne porte pas d’armes).";
    }else if (this.value == 6){
        txt = "<strong>Class attributs:</strong> +10 force, +5 volonté -5 charisme -5 éducation (Classe de nature, peut dompter les animaux. Possède une capacité : Forme animal. 0 définir au dés. Peut se changer une fois par jour)";
    }
    else if (this.value == 7){
        txt = "<strong>Class attributs:</strong> +10 constitution, +5 volonté, -10 dextérité, -5 force (apposition des mains, heal de 1d10).";
    }
    else if (this.value == 8){
        txt = "<strong>Class attributs:</strong> -15 force, +15 education, -10 volonté, +15 charisme, -5 constitution (Spécialiste des herbes et cataplasmes, peu crafter des remèdes, poisons etc …)";
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
            force.value = 15
        }else if(force.value == 15){
            force.value = force.value
        }

        if(education.value == 0 || education.value == null){
            education.value = -5
        }else if(education.value == -5){
            education.value = education.value
        }

        if(charism.value == 0 || charism.value == null){
            charism.value = -10
        }else if(charism.value == -10){
            charism.value = charism.value
        }
       
    }else if (raceID == 2){

        if(force.value == 0){
            force.value = -10
        }else if(force.value == -10){
            force.value = force.value
        }

        if(willpower.value == 0){
            willpower.value = 10
        }else if(willpower.value == 10){
            willpower.value = willpower.value
        }

        if(charism.value == 0){
            charism.value = 5
        }else if(education.value == 5){
            charism.value = charism.value
        }

    }else if (raceID == 3){

        if(dexterity.value == 0){
            dexterity.value = 10
        }else if(dexterity.value == +0){
            dexterity.value = dexterity.value
        }

        if(endurance.value == 0){
            endurance.value = -10
        }else if(endurance.value == -10){
            endurance.value = endurance.value
        }

    }else if (raceID == 4){

        if(willpower.value == 0){
            willpower.value = 5
        }else if(willpower.value == 5){
            willpower.value = willpower.value
        }

        if(endurance.value == 0){
            endurance.value = -5
        }else if(endurance.value == -5){
            endurance.value = endurance.value
        }

        if(education.value == 0){
            education.value = 5
        }else if(education.value == 5){
            education.value = education.value
        }

        if(force.value == 0){
            force.value = -5
        }else if(force.value == -5){
            force.value = force.value
        }

    }else if (raceID == 5){

        if(dexterity.value == 0){
            dexterity.value = -10
        }else if(dexterity.value == -10){
            dexterity.value = dexterity.value
        }

        if(endurance.value == 0){
            endurance.value = 5
        }else if(endurance.value == 5){
            endurance.value = endurance.value
        }

        if(charism.value == 0){
            charism.value = 5
        }else if(charism.value == 5){
            charism.value = charism.value
        }

        if(force.value == 0){
            force.value = -5
        }else if(force.value == -5){
            force.value = force.value
        }

    }else if (raceID == 6){

        if(dexterity.value == 0){
            dexterity.value = 5
        }else if(dexterity.value == 5){
            dexterity.value = dexterity.value
        }

        if(endurance.value == 0){
            endurance.value = -5
        }else if(endurance.value == -5){
            endurance.value = endurance.value
        }

        if(charism.value == 0){
            charism.value = -5
        }else if(charism.value == -5){
            charism.value = charism.value
        }

        if(perception.value == 0){
            perception.value = 5
        }else if(perception.value == 5){
            perception.value = perception.value
        }

        if(education.value == 0){
            education.value = -5
        }else if(education.value == -5){
            education.value = education.value
        }
    }

    if(classID == 1){

        if(force.value == 0 || force.value == null){
            force.value = 10
        }else if(force.value == 10){
            force.value = force.value
        }else{
            force.value =  parseInt(force.value) + 10;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = 10
        }else if(endurance.value == 10){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) + 10;
        }

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = -20
        }else if(willpower.value == -20){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) - 20;
        }

    }else if (classID == 2){

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = 20
        }else if(willpower.value == 20){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) + 20;
        }

        if(force.value == 0 || force.value == null){
            force.value = -20
        }else if(force.value == -20){
            force.value = force.value
        }else{
            force.value =  parseInt(force.value) -20;
        }

        if(education.value == 0 || education.value == null){
            education.value = 10
        }else if(education.value == 10){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) + 10;
        }

    }else if (classID == 3){

        if(dexterity.value == 0 || dexterity.value == null){
            dexterity.value = 20
        }else if(dexterity.value == 20){
            dexterity.value = dexterity.value
        }else{
            dexterity.value = parseInt(dexterity.value) + 20;
        }

        if(force.value == 0 || force.value == null){
            force.value = -15
        }else if(force.value == -15){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) - 15;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = -15
        }else if(endurance.value == -15){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) -15;
        }

    }else if (classID == 4){

        if(dexterity.value == 0 || dexterity.value == null){
            dexterity.value = +10
        }else if(dexterity.value == +10){
            dexterity.value = dexterity.value
        }else{
            dexterity.value = parseInt(dexterity.value) +10;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = -10
        }else if(endurance.value == -10){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) -10;
        }

        if(education.value == 0 || education.value == null){
            education.value = 5
        }else if(education.value == 5){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) +5;
        }

        if(perception.value == 0 || perception.value == null){
            perception.value = 5
        }else if(perception.value == 5){
            perception.value = perception.value
        }else{
            perception.value = parseInt(perception.value) +5;
        }

    }else if (classID == 5){

        if(force.value == 0 || force.value == null){
            force.value = 5
        }else if(force.value == 5){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) +5;
        }

        if(dexterity.value == 0 || dexterity.value == null){
            dexterity.value = 5
        }else if(dexterity.value == 5){
            dexterity.value = dexterity.value
        }else{
            dexterity.value = parseInt(dexterity.value) +5;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = 5
        }else if(endurance.value == 5){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) +5;
        }

        if(education.value == 0 || education.value == null){
            education.value = 5
        }else if(education.value == 5){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) +5;
        }

        if(charism.value == 0 || charism.value == null){
            charism.value = -15
        }else if(charism.value == -15){
            charism.value = charism.value
        }else{
            charism.value = parseInt(charism.value) -15;
        }

    }else if (classID == 6){

        if(force.value == 0 || force.value == null){
            force.value = 10
        }else if(force.value == 10){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) +10;
        }

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = 5
        }else if(willpower.value == 5){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) +5;
        }

        if(charism.value == 0 || charism.value == null){
            charism.value = -5
        }else if(charism.value == -5){
            charism.value = charism.value
        }else{
            charism.value = parseInt(charism.value) -5;
        }

        if(education.value == 0 || education.value == null){
            education.value = -5
        }else if(education.value == -5){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) -5;
        }

    }
    else if (classID == 7){
        if(endurance.value == 0 || endurance.value == null){
            endurance.value = 10
        }else if(endurance.value == 10){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) +10;
        }

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = 5
        }else if(willpower.value == 5){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) +5;
        }

        if(dexterity.value == 0 || dexterity.value == null){
            dexterity.value = -10
        }else if(dexterity.value == -10){
            dexterity.value = dexterity.value
        }else{
            dexterity.value = parseInt(dexterity.value) -10;
        }

        if(force.value == 0 || force.value == null){
            force.value = -5
        }else if(force.value == -5){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) -5;
        }
    }
    else if (classID == 8){

        if(force.value == 0 || force.value == null){
            force.value = -15
        }else if(force.value == -15){
            force.value = force.value
        }else{
            force.value = parseInt(force.value) -15;
        }

        if(education.value == 0 || education.value == null){
            education.value = 15
        }else if(education.value == 15){
            education.value = education.value
        }else{
            education.value = parseInt(education.value) +15;
        }

        if(willpower.value == 0 || willpower.value == null){
            willpower.value = -10
        }else if(willpower.value == -10){
            willpower.value = willpower.value
        }else{
            willpower.value = parseInt(willpower.value) -10;
        }

        if(charism.value == 0 || charism.value == null){
            charism.value = 15
        }else if(charism.value == 15){
            charism.value = charism.value
        }else{
            charism.value = parseInt(charism.value) +15;
        }

        if(endurance.value == 0 || endurance.value == null){
            endurance.value = -5
        }else if(endurance.value == -5){
            endurance.value = endurance.value
        }else{
            endurance.value = parseInt(endurance.value) -5;
        }
    }


});
