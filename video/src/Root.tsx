import {Composition} from 'remotion';
import {Hello} from './Hello';

export const Root: React.FC = () => (
  <Composition id="Hello" component={Hello} durationInFrames={90} fps={30} width={1280} height={720} />
);
