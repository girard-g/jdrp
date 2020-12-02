require('vue')

let a = window.location.href.split("/");
    const USERID = a[a.length -1];

      new Vue({
      el: '#base-player-stats',
      data () {
        return {
          name:null,
          race:null,
          carcter_class:null,
          level:null,
          reputation:null,
          particularity:null,
          alignment:null,
          strengh:null,
          dexterity:null,
          luck:null,
          willpower:null,
          endurance:null,
          charism:null,
          perception:null,
          education:null,
          weapon:null,
          distance_weapon:null,
          bare_hand:null,
          armor:null,
          loading: true,
        }
      },
      mounted () {
        axios
          .get('/users/player-stats/' + USERID)
          .then(
            response => (
              this.name = response.data.name,
              this.race = response.data.race,
              this.carcter_class = response.data.class,
              this.level = response.data.level,
              this.reputation = response.data.reputation,
              this.particularity = response.data.particularity,
              this.alignment = response.data.alignment,
              this.strengh = response.data.strengh,
              this.dexterity = response.data.dexterity,
              this.luck = response.data.luck,
              this.willpower = response.data.willpower,
              this.endurance = response.data.endurance,
              this.charism = response.data.charism,
              this.perception = response.data.perception,
              this.education = response.data.education,
              this.weapon = response.data.weapon,
              this.distance_weapon = response.data.distance_weapon,
              this.bare_hand = response.data.bare_hand,
              this.armor = response.data.armor
              ))
          .catch(error => console.log(error))
          .finally(() => this.loading = false)
      }
    })