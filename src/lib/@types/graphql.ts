export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type Amplifier = {
  __typename?: 'Amplifier';
  analog_connections?: Maybe<Array<AnalogConn>>;
  max_sample_rate: SampleRate;
  midi?: Maybe<MidiType>;
  network_connections?: Maybe<Array<NetworkConn>>;
  power: Power;
  signal_protocol?: Maybe<Protocol>;
  total_inputs: Scalars['Int']['output'];
  total_ouputs: Scalars['Int']['output'];
  word_clock: Scalars['Boolean']['output'];
};

export type AmplifierInput = {
  analog_connections?: InputMaybe<Array<AnalogConnInput>>;
  max_sample_rate: SampleRate;
  midi?: InputMaybe<MidiType>;
  network_connections?: InputMaybe<Array<NetworkConnInput>>;
  power: PowerInput;
  signal_protocol?: InputMaybe<Protocol>;
  total_inputs: Scalars['Int']['input'];
  total_ouputs: Scalars['Int']['input'];
  word_clock: Scalars['Boolean']['input'];
};

export enum Analog {
  Bnc = 'BNC',
  Dc_12V = 'DC_12V',
  DualPinPhoenix = 'DUAL_PIN_PHOENIX',
  Nl2 = 'NL2',
  Nl4 = 'NL4',
  Nl8 = 'NL8',
  TriPinPhoenix = 'TRI_PIN_PHOENIX',
  Trrs = 'TRRS',
  Trs = 'TRS',
  Ts = 'TS',
  XlrAnalog = 'XLR_ANALOG',
  XlrDigital = 'XLR_DIGITAL',
  XlrQuarterCombo = 'XLR_QUARTER_COMBO'
}

export type AnalogConn = {
  __typename?: 'AnalogConn';
  port_id: Scalars['String']['output'];
  port_kind: Analog;
  port_usage: AnalogUsage;
  signal_lines: Scalars['Int']['output'];
};

export type AnalogConnInput = {
  port_id: Scalars['String']['input'];
  port_kind?: Analog;
  port_usage?: AnalogUsage;
  signal_lines?: Scalars['Int']['input'];
};

export enum AnalogUsage {
  Aux = 'AUX',
  Input = 'INPUT',
  Input_48V = 'INPUT_48V',
  Output = 'OUTPUT'
}

export enum Category {
  Amplifier = 'AMPLIFIER',
  Computer = 'COMPUTER',
  Console = 'CONSOLE',
  Generic = 'GENERIC',
  Microphones = 'MICROPHONES',
  Monitoring = 'MONITORING',
  Network = 'NETWORK',
  Processor = 'PROCESSOR',
  Receiver = 'RECEIVER',
  Speaker = 'SPEAKER',
  SpkHardware = 'SPK_HARDWARE',
  Stagebox = 'STAGEBOX',
  Transmitter = 'TRANSMITTER'
}

export type CategoryDetails = Amplifier | Computer | Console | Microphone | Monitoring | Processor | Rx | Speaker | StageBox | Tx;

export type CategoryDetailsInput = {
  amplifier_input?: InputMaybe<AmplifierInput>;
  computer_input?: InputMaybe<ComputerInput>;
  console_input?: InputMaybe<ConsoleInput>;
  microphone_input?: InputMaybe<MicrophoneInput>;
  monitoring_input?: InputMaybe<MonitoringInput>;
  processor_input?: InputMaybe<ProcessorInput>;
  rx_input?: InputMaybe<RxInput>;
  speaker_input?: InputMaybe<SpeakerInput>;
  stagebox_input?: InputMaybe<StageBoxInput>;
  tx_input?: InputMaybe<TxInput>;
};

export type Computer = {
  __typename?: 'Computer';
  computer_ports?: Maybe<Array<Maybe<ComputerConn>>>;
  cpu_processor: Scalars['String']['output'];
  dedicated_graphics?: Maybe<Scalars['Boolean']['output']>;
  model_year: Scalars['Int']['output'];
  network_connections?: Maybe<Array<Maybe<NetworkConn>>>;
  operating_system: Scalars['String']['output'];
  power: Power;
  ram_size: Scalars['Int']['output'];
  total_storage: Scalars['Int']['output'];
};

export type ComputerConn = {
  __typename?: 'ComputerConn';
  front_port: Scalars['Boolean']['output'];
  port_id?: Maybe<Scalars['String']['output']>;
  port_kind: ComputerConnKind;
};

export type ComputerConnInput = {
  front_port?: Scalars['Boolean']['input'];
  port_id?: InputMaybe<Scalars['String']['input']>;
  port_kind: ComputerConnKind;
};

export enum ComputerConnKind {
  Displayport = 'DISPLAYPORT',
  Firewire = 'FIREWIRE',
  Hdmi = 'HDMI',
  MiniDisplayport = 'MINI_DISPLAYPORT',
  MiniHdmi = 'MINI_HDMI',
  MircoB = 'MIRCO_B',
  SdCard = 'SD_CARD',
  UsbA = 'USB_A',
  UsbB = 'USB_B',
  UsbC = 'USB_C',
  UsbCThunderbolt = 'USB_C_THUNDERBOLT'
}

export type ComputerInput = {
  computer_ports?: InputMaybe<Array<ComputerConnInput>>;
  cpu_processor: Scalars['String']['input'];
  dedicated_graphics: Scalars['Boolean']['input'];
  model_year: Scalars['Int']['input'];
  network_connections?: InputMaybe<Array<NetworkConnInput>>;
  operating_system: Scalars['String']['input'];
  power: PowerInput;
  ram_size: Scalars['Int']['input'];
  total_storage: Scalars['Int']['input'];
};

export type Console = {
  __typename?: 'Console';
  analog_connections?: Maybe<Array<AnalogConn>>;
  can_expand: Scalars['Boolean']['output'];
  faders: Scalars['Int']['output'];
  max_sample_rate: SampleRate;
  midi: MidiType;
  motorized: Scalars['Boolean']['output'];
  network_connections?: Maybe<Array<NetworkConn>>;
  power: Power;
  protocol_inputs: Scalars['Int']['output'];
  signal_protocol: Protocol;
  word_clock: Scalars['Boolean']['output'];
};

export type ConsoleInput = {
  analog_connections?: InputMaybe<Array<AnalogConnInput>>;
  can_expand: Scalars['Boolean']['input'];
  faders: Scalars['Int']['input'];
  max_sample_rate: SampleRate;
  midi: MidiType;
  motorized: Scalars['Boolean']['input'];
  network_connections?: InputMaybe<Array<NetworkConnInput>>;
  power?: InputMaybe<PowerInput>;
  protocol_inputs: Scalars['Int']['input'];
  signal_protocol: Protocol;
  word_clock: Scalars['Boolean']['input'];
};

export enum DiaphagmSize {
  Large = 'LARGE',
  Med = 'MED',
  Small = 'SMALL'
}

export type Dimension = {
  __typename?: 'Dimension';
  height: Scalars['Float']['output'];
  length: Scalars['Float']['output'];
  width: Scalars['Float']['output'];
};

export type DimensionInput = {
  height: Scalars['Float']['input'];
  length: Scalars['Float']['input'];
  width: Scalars['Float']['input'];
};

export type Error = {
  __typename?: 'Error';
  field: Scalars['String']['output'];
  message: Scalars['String']['output'];
};

export type Item = {
  __typename?: 'Item';
  category: Category;
  created_at: Scalars['String']['output'];
  details?: Maybe<CategoryDetails>;
  dimensions?: Maybe<Dimension>;
  keywords?: Maybe<Array<Scalars['String']['output']>>;
  manufacturer: Scalars['String']['output'];
  model: Scalars['String']['output'];
  notes?: Maybe<Scalars['String']['output']>;
  pdf_blob?: Maybe<Scalars['String']['output']>;
  updated_at: Scalars['String']['output'];
  weight: Scalars['Float']['output'];
};

export type ItemInput = {
  category?: Category;
  dimensions?: InputMaybe<DimensionInput>;
  keywords?: InputMaybe<Array<Scalars['String']['input']>>;
  manufacturer: Scalars['String']['input'];
  model: Scalars['String']['input'];
  notes?: InputMaybe<Scalars['String']['input']>;
  pdf_blob?: InputMaybe<Scalars['String']['input']>;
  weight: Scalars['Float']['input'];
};

export type Microphone = {
  __typename?: 'Microphone';
  connector?: Maybe<Analog>;
  diaphragm_size?: Maybe<Scalars['Float']['output']>;
  low_cut?: Maybe<Scalars['Boolean']['output']>;
  max_spl: Scalars['Float']['output'];
  microphone_type: MicrophoneType;
  output_impedance?: Maybe<Scalars['Float']['output']>;
  pad?: Maybe<Scalars['Boolean']['output']>;
  pattern: Array<Maybe<PolarPattern>>;
  phantom?: Maybe<Scalars['Boolean']['output']>;
};

export type MicrophoneInput = {
  connector?: InputMaybe<Analog>;
  diaphragm_size?: InputMaybe<Scalars['Float']['input']>;
  low_cut?: InputMaybe<Scalars['Boolean']['input']>;
  max_spl: Scalars['Float']['input'];
  microphone_type: MicrophoneType;
  output_impedance?: InputMaybe<Scalars['Float']['input']>;
  pad?: InputMaybe<Scalars['Boolean']['input']>;
  pattern: Array<InputMaybe<PolarPattern>>;
  phantom?: InputMaybe<Scalars['Boolean']['input']>;
};

export enum MicrophoneType {
  Condensor = 'CONDENSOR',
  Dynamic = 'DYNAMIC',
  PrePoloraizedCondensor = 'PRE_POLORAIZED_CONDENSOR',
  Ribbon = 'RIBBON'
}

export enum MidiType {
  Serial = 'SERIAL',
  Usb = 'USB'
}

export type Monitoring = {
  __typename?: 'Monitoring';
  analog_connections?: Maybe<Array<AnalogConn>>;
  computer_ports?: Maybe<Array<ComputerConn>>;
  distro: Scalars['Boolean']['output'];
  network_connections?: Maybe<Array<NetworkConn>>;
  power: Power;
  signal_protocol: Protocol;
};

export type MonitoringInput = {
  analog_connections?: InputMaybe<Array<AnalogConnInput>>;
  computer_ports?: InputMaybe<Array<ComputerConnInput>>;
  distro: Scalars['Boolean']['input'];
  network_connections?: InputMaybe<Array<NetworkConnInput>>;
  power: PowerInput;
  signal_protocol: Protocol;
};

export type Mutation = {
  __typename?: 'Mutation';
  createItem: Item;
  createUser: User;
};


export type MutationCreateItemArgs = {
  details?: InputMaybe<CategoryDetailsInput>;
  input: ItemInput;
};


export type MutationCreateUserArgs = {
  input: UserInput;
};

export enum NetworkCableKind {
  Cat_5e = 'CAT_5e',
  Cat_6a = 'CAT_6a',
  Cat_7 = 'CAT_7',
  Fiber = 'FIBER'
}

export type NetworkConn = {
  __typename?: 'NetworkConn';
  max_conn_speed: NetworkSpeed;
  poe?: Maybe<Scalars['Boolean']['output']>;
  port_id?: Maybe<Scalars['String']['output']>;
  protocol: Protocol;
};

export type NetworkConnInput = {
  max_conn_speed: NetworkSpeed;
  poe?: InputMaybe<Scalars['Boolean']['input']>;
  port_id?: InputMaybe<Scalars['String']['input']>;
  protocol: Protocol;
};

export enum NetworkSpeed {
  Gigabit = 'GIGABIT',
  Superspeed = 'SUPERSPEED',
  TenGigabit = 'TEN_GIGABIT'
}

export enum NetworkType {
  AccessPoint = 'ACCESS_POINT',
  Injector = 'INJECTOR',
  Modem = 'MODEM',
  NetworkBridge = 'NETWORK_BRIDGE',
  Nic = 'NIC',
  Repeater = 'REPEATER',
  Router = 'ROUTER',
  RouterSwAp = 'ROUTER_SW_AP',
  SwitchManaged = 'SWITCH_MANAGED',
  SwitchUnmanaged = 'SWITCH_UNMANAGED'
}

export enum PolarPattern {
  Cardioid = 'CARDIOID',
  Figure_8 = 'FIGURE_8',
  HalfCardioid = 'HALF_CARDIOID',
  Hypercardioid = 'HYPERCARDIOID',
  Omni = 'OMNI',
  Supercardioid = 'SUPERCARDIOID'
}

export type Power = {
  __typename?: 'Power';
  input_connector?: Maybe<PowerConnector>;
  lower_voltage?: Maybe<Scalars['Float']['output']>;
  max_wattage: Scalars['Float']['output'];
  output_connector?: Maybe<PowerConnector>;
  redundant?: Maybe<Scalars['Boolean']['output']>;
  upper_voltage?: Maybe<Scalars['Float']['output']>;
  wattage: Scalars['Float']['output'];
};

export enum PowerConnector {
  Edison = 'EDISON',
  Edison_20A = 'EDISON_20A',
  Iec = 'IEC',
  L6_20 = 'L6_20',
  L6_30 = 'L6_30',
  L6_50 = 'L6_50',
  L6_60 = 'L6_60',
  Powercon_20A = 'POWERCON_20A',
  Powercon_32A = 'POWERCON_32A',
  PowerconTrue1 = 'POWERCON_TRUE1',
  PowerconTrue1Top = 'POWERCON_TRUE1_TOP'
}

export type PowerInput = {
  input_connector?: InputMaybe<PowerConnector>;
  lower_voltage?: InputMaybe<Scalars['Float']['input']>;
  max_wattage: Scalars['Float']['input'];
  output_connector?: InputMaybe<PowerConnector>;
  redundant?: InputMaybe<Scalars['Boolean']['input']>;
  upper_voltage?: InputMaybe<Scalars['Float']['input']>;
  wattage: Scalars['Float']['input'];
};

export type Processor = {
  __typename?: 'Processor';
  analog_connections?: Maybe<Array<AnalogConn>>;
  max_sample_rate: SampleRate;
  midi?: Maybe<MidiType>;
  network_connections?: Maybe<Array<NetworkConn>>;
  power: Power;
  signal_protocol: Protocol;
  total_inputs: Scalars['Int']['output'];
  total_ouputs: Scalars['Int']['output'];
};

export type ProcessorInput = {
  analog_connections?: InputMaybe<Array<AnalogConnInput>>;
  max_sample_rate: SampleRate;
  midi?: InputMaybe<MidiType>;
  network_connections?: InputMaybe<Array<NetworkConnInput>>;
  power: PowerInput;
  signal_protocol: Protocol;
  total_inputs: Scalars['Int']['input'];
  total_ouputs: Scalars['Int']['input'];
};

export enum Protocol {
  Aes_67 = 'AES_67',
  Avb = 'AVB',
  AvbMilan = 'AVB_MILAN',
  ANet = 'A_NET',
  Dante = 'DANTE',
  Ip = 'IP',
  Optocore = 'OPTOCORE',
  Ultranet = 'ULTRANET'
}

export type Query = {
  __typename?: 'Query';
  find_by_id: Item;
  find_by_model: Item;
  fuzzy_by_model: Array<Maybe<Item>>;
  items: Array<Item>;
  users: Array<User>;
};


export type QueryFind_By_IdArgs = {
  id: Scalars['ID']['input'];
};


export type QueryFind_By_ModelArgs = {
  model_name: Scalars['String']['input'];
};


export type QueryFuzzy_By_ModelArgs = {
  model_name: Scalars['String']['input'];
};

export type Rx = {
  __typename?: 'Rx';
  analog_connections?: Maybe<Array<AnalogConn>>;
  computer_ports?: Maybe<Array<ComputerConn>>;
  digital: Scalars['Boolean']['output'];
  lower_rf_range: Scalars['Float']['output'];
  network_connections?: Maybe<Array<NetworkConn>>;
  num_of_rxs: Scalars['Int']['output'];
  power: Power;
  range: Scalars['Int']['output'];
  signal_protocol?: Maybe<Protocol>;
  upper_rf_range: Scalars['Float']['output'];
};

export type RxInput = {
  analog_connections?: InputMaybe<Array<AnalogConnInput>>;
  computer_ports?: InputMaybe<Array<ComputerConnInput>>;
  digital: Scalars['Boolean']['input'];
  lower_rf_range: Scalars['Float']['input'];
  network_connections?: InputMaybe<Array<NetworkConnInput>>;
  num_of_rxs: Scalars['Int']['input'];
  power: PowerInput;
  range: Scalars['Int']['input'];
  signal_protocol?: InputMaybe<Protocol>;
  upper_rf_range: Scalars['Float']['input'];
};

export enum SampleRate {
  Hd = 'HD',
  Sd = 'SD',
  Uhd = 'UHD'
}

export type Speaker = {
  __typename?: 'Speaker';
  active: Scalars['Boolean']['output'];
  analog_connections?: Maybe<Array<AnalogConn>>;
  drivers: Array<SpeakerDriver>;
  high_freq_resp: Scalars['Float']['output'];
  line_array?: Maybe<Scalars['Boolean']['output']>;
  low_freq_resp: Scalars['Float']['output'];
  max_spl: Scalars['Float']['output'];
  network_connections?: Maybe<Array<NetworkConn>>;
  power: Power;
  processing: Scalars['Boolean']['output'];
  subwoofer?: Maybe<Scalars['Boolean']['output']>;
};

export enum SpeakerDriver {
  Midrange = 'MIDRANGE',
  Subwoofer = 'SUBWOOFER',
  Tweeter = 'TWEETER',
  Woofer = 'WOOFER'
}

export type SpeakerInput = {
  active: Scalars['Boolean']['input'];
  analog_connections?: InputMaybe<Array<AnalogConnInput>>;
  drivers: Array<SpeakerDriver>;
  high_freq_resp: Scalars['Float']['input'];
  line_array?: InputMaybe<Scalars['Boolean']['input']>;
  low_freq_resp: Scalars['Float']['input'];
  max_spl: Scalars['Float']['input'];
  network_connections?: InputMaybe<Array<NetworkConnInput>>;
  power: PowerInput;
  processing: Scalars['Boolean']['input'];
  subwoofer?: InputMaybe<Scalars['Boolean']['input']>;
};

export type StageBox = {
  __typename?: 'StageBox';
  analog_connections?: Maybe<Array<AnalogConn>>;
  max_sample_rate: SampleRate;
  network_connections?: Maybe<Array<NetworkConn>>;
  power: Power;
  signal_protocol: Protocol;
  total_inputs: Scalars['Int']['output'];
  total_ouputs: Scalars['Int']['output'];
  word_clock: Scalars['Boolean']['output'];
};

export type StageBoxInput = {
  analog_connections?: InputMaybe<Array<AnalogConnInput>>;
  max_sample_rate: SampleRate;
  network_connections?: InputMaybe<Array<NetworkConnInput>>;
  power: PowerInput;
  signal_protocol: Protocol;
  total_inputs: Scalars['Int']['input'];
  total_ouputs: Scalars['Int']['input'];
  word_clock: Scalars['Boolean']['input'];
};

export enum TxForm {
  Bodypack = 'BODYPACK',
  Handheld = 'HANDHELD'
}

export enum TransmitterConnector {
  Microdot = 'MICRODOT',
  ShureTa4 = 'SHURE_TA4',
  TriPin = 'TRI_PIN',
  Trrs = 'TRRS'
}

export type Tx = {
  __typename?: 'Tx';
  connector?: Maybe<TransmitterConnector>;
  form: TxForm;
  lower_rf_range: Scalars['Float']['output'];
  mute: Scalars['Boolean']['output'];
  power_type: Scalars['String']['output'];
  range: Scalars['Int']['output'];
  upper_rf_range: Scalars['Float']['output'];
};

export type TxInput = {
  connector?: InputMaybe<TransmitterConnector>;
  form: TxForm;
  lower_rf_range: Scalars['Float']['input'];
  mute: Scalars['Boolean']['input'];
  power_type: Scalars['String']['input'];
  range: Scalars['Int']['input'];
  upper_rf_range: Scalars['Float']['input'];
};

export type User = {
  __typename?: 'User';
  email: Scalars['String']['output'];
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
};

export type UserInput = {
  email: Scalars['String']['input'];
  name: Scalars['String']['input'];
};
