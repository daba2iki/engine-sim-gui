import { darkTheme, lightTheme } from 'naive-ui'

// ライトテーマのカスタム設定
export const customLightTheme = {
  ...lightTheme,
  common: {
    primaryColor: '#FFC81E',
    primaryColorHover: '#FFD54A',
    primaryColorPressed: '#E6B000',
    primaryColorSuppl: '#FFC81E'
  },
  Button: {
    color: '#FFFFFF',
    colorHover: '#F0F0F0'
  }
}

// ダークテーマのカスタム設定
export const customDarkTheme = {
  ...darkTheme,
  common: {
    primaryColor: '#FFC81E',
    primaryColorHover: '#FFD54A',
    primaryColorPressed: '#E6B000',
    primaryColorSuppl: '#FFC81E'
  },
  Button: {
    color: '#FFFFFF',
    colorHover: '#F0F0F0'
  }
}