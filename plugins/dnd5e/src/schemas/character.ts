interface AbilityScore {
  score: number;
  modifier: number;
}

interface SavingThrow {
  modifier: number;
  proficiency: boolean;
}

interface Skill {
  modifier: number;
  proficiency: boolean;
}

interface Lore {
  name: string;
  race: string;
  class: string;
  background: string;
  alignment: Enumerator;
  gender: string;
  personality: string[];
  ideals: string[];
  bonds: string[];
  flaws: string[];
  traits: string[];
}

interface Character {
  name: string;
  level: number;
  playername: string;
  experience: number;
  hitPoints: number;
  tempHitPoints: number;
  armorClass: number;
  initiative: number;
  speed: number;
  lore: Lore;
  skills: Skill[];

  savingThrows: {
    strength: SavingThrow;
    dexterity: SavingThrow;
    constitution: SavingThrow;
    intelligence: SavingThrow;
    wisdom: SavingThrow;
    charisma: SavingThrow;
  };

  abilities: {
    strength: AbilityScore;
    dexterity: AbilityScore;
    constitution: AbilityScore;
    intelligence: AbilityScore;
    wisdom: AbilityScore;
    charisma: AbilityScore;
  };
}