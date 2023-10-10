import { writable } from "svelte/store"
import type { Gear, Cable } from "$lib/@types/equipment"
import { Analog, Category } from "$lib/@types/graphql";

export const cableList = writable<Cable[]>([
  {
    id: 0,
    data: {
      name: "Stage Mic",
      description: "Lead Vox",
      bundleName: "", 
      model: "XLR",
      cableKind: "Analog",
      length: 25,
      connectionName: "",
      destinationName: ""
    },
    metadata: {
      textColor: 'white',
      color: {r: 0, g: 0, b: 255},
      alignment: "Left"
    },
    source: { name: "", kind: Analog.XlrDigital },
    destination: { name: "", kind: Analog.XlrDigital },
    bundle: { name: "", cableIds: []},
  }
]);

export const gearList = writable<Gear[]>([
  {
    equipment: {
      _id: "",
      category: Category.Console,
      model: "",
      cost: 6000,
      wattage: 15.3,
      details: {} 
    },  
    items: [
      { id: 0, description: "Primary", quantity: 0, purpose: "" },
    ]
  },
]);
