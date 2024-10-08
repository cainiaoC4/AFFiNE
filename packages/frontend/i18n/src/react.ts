import { type ComponentType, useMemo } from 'react';
import { Trans as ReactTrans, useTranslation } from 'react-i18next';

import { type TypedTrans } from './i18n-generated';
import { createI18nWrapper } from './i18next';

type TypedTrans = typeof TypedTrans;

type TransProps<Key extends keyof TypedTrans> =
  TypedTrans[Key] extends ComponentType<infer Props> ? Props : any;

export const useI18n = () => {
  const { i18n } = useTranslation('translation');

  return useMemo(() => createI18nWrapper(() => i18n), [i18n]);
};

export { I18nextProvider } from 'react-i18next';

export function Trans<T extends keyof TypedTrans>(
  props: TransProps<T> & { i18nKey: T }
) {
  return ReactTrans(props as any);
}
