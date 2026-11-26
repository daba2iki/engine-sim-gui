export type ParamType = 'bool' | 'number' | 'string' | 'select'

export interface Param {
  key: string
  label: string
  type: ParamType
  value: boolean | number | string
  default: boolean | number | string
  unit?: string | null
  description?: string
  category?: string
  options?: string[] | null
}

export interface EngineDoc {
  name?: string
  parameters: Param[]
}
