import type { Cable, Gear } from "./equipment"

export type ProductionInformation = {
    productionName: string
    director: string
    venue: string
    showImage?: string 
    designerStamp?: string
    notes?: string
}

export type TeamRole = { 
  role: string,
  name: string,
  email: string,
  phone: string 
}


export type Input = {
    channel: number | null;
    input_description: string | null;
    input_device: string | null;
    note: string | null;
}

export type Output = {
    channel: number | null;
    output_name: string | null;
    output_device: string | null;
    destination: string | null;
}

export type IO = {
    inputs: Input[];
    outputs: Output[];
};

export type Project = {
  prodInfo: ProductionInformation
  audioTeam: TeamRole[]
  ioList: IO
  gearList: Gear[]
  cableList: Cable[]
  meta: Meta
}

export type Meta = {
  currentFilePath: string,
}
