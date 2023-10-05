import type { Analog, Category, ComputerConnKind, NetworkType, PowerConnector } from "./graphql"
import type { Bundle } from "./flow"

// How details of a singular Gear Item
export type Item = {
    id: number
    description: string
    quantity: number,
    purpose: string,
    publicNotes: string,
    privateNotes: string,
  }

export type Gear = {
    equipment: Equip
    items: Item[]
}

export type Equip = {
  _id: string,
  createdAt: string,
  updatedAt: string,
  cost: number,
  model: string,
  weight: number,
  wattage: number,
  manufacturer: string,
  dimensions: { length: number, width: number, height: number},
  notes: string,
  category: Category,
  details: any | null,
}


type CableData = {
  name: string,
  description: string,
  model: string,
  length: number,
  destinationName: string | null,
  connectionName: string | null,
  cableKind: "Power" | "Analog" | "Digital"
  bundleName: string | null, 
}

type CableMeta = {
  textColor: 'black' | "white"
  color: {r: number, g: number, b: number },
  alignment: "Left" | "Right" | "Center" 
}

export type Cable = {
  id: number,
  data: CableData,
  metadata: CableMeta,
  source: Connection,
  destination: Connection , // Same as Source
  bundle: Bundle
}


export type Connection = {
  name: string,
  kind: Analog | PowerConnector | NetworkType, 
}


