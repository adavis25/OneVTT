interface AbilityScore {
  score: number;
  modifier: number;
}

interface Character {
  name: string;
  level: number;
  race: string;
  class: string;
  background: string;
  alignment: string;
  experience: number;
  hitPoints: number;
  armorClass: number;
  speed: number;

  abilities: {
    strength: AbilityScore;
    dexterity: AbilityScore;
    constitution: AbilityScore;
    intelligence: AbilityScore;
    wisdom: AbilityScore;
    charisma: AbilityScore;
  };
}