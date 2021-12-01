import Commands from '../commands';

export const cap = 'C';

export const commands = Commands.comingSoon;

export const Visual = props => (
  <svg xmlns='http://www.w3.org/2000/svg'>
    <path d='M 36 16 h 24' stroke-width='2' />
    <path d='M 60 16 l -5 -5' stroke-width='2' />
    <path d='M 60 16 l -5 5' stroke-width='2' />
    <path d='M 36 30 h 24' stroke-width='2' />
    <path d='M 36 30 l 5 5' stroke-width='2' />
    <path d='M 36 30 l 5 -5' stroke-width='2' />
  </svg>
);

export const Help = ({ Block }) => (
  <>
    <Block>
      <b>PLUGIN</b> mode doesn't do anything yet.
    </Block>
  </>
);
