import {AbsoluteFill, interpolate, useCurrentFrame} from 'remotion';

export const Hello: React.FC = () => {
  const frame = useCurrentFrame();
  const opacity = interpolate(frame, [0, 30], [0, 1], {extrapolateRight: 'clamp'});
  return (
    <AbsoluteFill style={{background: '#0b1020', color: 'white', justifyContent: 'center', alignItems: 'center', fontSize: 96, opacity}}>
      Consensus
    </AbsoluteFill>
  );
};
