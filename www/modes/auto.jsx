import Actions from '../actions';

export const cap = 'X';

export const actions = Actions.comingSoon;

export const Visual = props => {
  const ls = Array
    .from({ length: 50 })
    .map((_, i, a) => `l 1 ${Math.cos(i/a.length*2*Math.PI)}`)
    .join('');
  return (
    <svg xmlns='http://www.w3.org/2000/svg'>
      <path d={`M 23 23 ${ls}`} stroke-width='2'></path>
    </svg>
  );
};

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>AUTO</b> mode doesn't do anything yet.
    </Block>
  </>
);
