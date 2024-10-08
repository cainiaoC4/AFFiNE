import type { QuickSearchItem } from '@affine/core/modules/quicksearch';
import { HighlightText } from '@affine/core/modules/quicksearch/views/highlight-text';
import { useI18n } from '@affine/i18n';

export interface SearchResLabelProps {
  item: QuickSearchItem;
}
export const SearchResLabel = ({ item }: SearchResLabelProps) => {
  const i18n = useI18n();

  return (
    <HighlightText
      text={i18n[
        typeof item.label === 'string' ? item.label : item.label.title
      ]()}
      start="<b>"
      end="</b>"
    />
  );
};
