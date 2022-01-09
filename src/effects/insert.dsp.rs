

pub struct insertSIG0 {
	iVec8: [i32;2],
	iRec27: [i32;2],
}

impl insertSIG0 {
	
	fn get_num_inputsinsertSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsinsertSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initinsertSIG0(&mut self, sample_rate: i32) {
		let mut l29: i32 = 0;
		loop {
			self.iVec8[l29 as usize] = 0;
			l29 = (l29 + 1);
			if (l29 < 2) { continue; } else { break; }
		}
		let mut l30: i32 = 0;
		loop {
			self.iRec27[l30 as usize] = 0;
			l30 = (l30 + 1);
			if (l30 < 2) { continue; } else { break; }
		}
	}
	
	fn fillinsertSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec8[0] = 1;
			self.iRec27[0] = ((self.iVec8[1] + self.iRec27[1]) % 65536);
			table[i1 as usize] = F32::sin((9.58738019e-05 * (self.iRec27[0] as F32)));
			self.iVec8[1] = self.iVec8[0];
			self.iRec27[1] = self.iRec27[0];
		}
	}

}


pub fn newinsertSIG0() -> insertSIG0 { 
	insertSIG0 {
		iVec8: [0;2],
		iRec27: [0;2],
	}
}
fn insert_faustpower2_f(value: F32) -> F32 {
	return (value * value);
}
static mut ftbl0insertSIG0: [F32;65536] = [0.0;65536];
pub struct insert {
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fButton0: F32,
	fVec1: [F32;2],
	iRec7: [i32;2],
	fConst2: F32,
	fEntry0: F32,
	iVec2: [i32;2],
	fEntry1: F32,
	fRec6: [F32;2],
	fEntry2: F32,
	fButton1: F32,
	fEntry3: F32,
	fRec9: [F32;2],
	fRec8: [F32;2],
	IOTA: i32,
	fVec3: [F32;131072],
	fRec10: [F32;2],
	fEntry4: F32,
	fRec11: [F32;2],
	fEntry5: F32,
	fRec12: [F32;2],
	fEntry6: F32,
	fRec13: [F32;2],
	fEntry7: F32,
	fRec14: [F32;2],
	iRec15: [i32;2],
	fRec16: [F32;2],
	fEntry8: F32,
	fRec17: [F32;2],
	fEntry9: F32,
	fRec18: [F32;2],
	fConst3: F32,
	fRec19: [F32;2],
	fConst4: F32,
	fEntry10: F32,
	fRec21: [F32;2],
	fEntry11: F32,
	fRec22: [F32;2],
	fRec20: [F32;2],
	fVec4: [F32;2],
	fVec5: [F32;4096],
	fConst5: F32,
	iRec23: [i32;2],
	fRec24: [F32;2],
	fVec6: [F32;2],
	fVec7: [F32;4096],
	fRec25: [F32;2],
	fRec26: [F32;2],
	fConst6: F32,
	fRec28: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fRec29: [F32;2],
	fRec30: [F32;2],
	fRec32: [F32;2],
	fRec33: [F32;2],
	fRec34: [F32;2],
	fEntry12: F32,
	fRec36: [F32;2],
	fEntry13: F32,
	fRec37: [F32;2],
	fRec38: [F32;2],
	fRec39: [F32;2],
	fEntry14: F32,
	fRec41: [F32;2],
	fRec40: [F32;2],
	fRec43: [F32;2],
	fVec9: [F32;2],
	fVec10: [F32;4096],
	fRec42: [F32;2],
	fRec44: [F32;2],
	fRec46: [F32;2],
	fRec48: [F32;2],
	fVec11: [F32;2],
	fVec12: [F32;4096],
	fRec47: [F32;2],
	fRec49: [F32;2],
	fRec51: [F32;2],
	fEntry15: F32,
	fRec52: [F32;2],
	fEntry16: F32,
	fRec53: [F32;2],
	fRec54: [F32;2],
	fRec55: [F32;2],
	fEntry17: F32,
	fRec57: [F32;2],
	fRec56: [F32;2],
	fRec59: [F32;2],
	fVec13: [F32;2],
	fVec14: [F32;4096],
	fRec58: [F32;2],
	fRec60: [F32;2],
	fRec62: [F32;2],
	fRec64: [F32;2],
	fVec15: [F32;2],
	fVec16: [F32;4096],
	fRec63: [F32;2],
	fRec65: [F32;2],
	fRec67: [F32;2],
	fVec17: [F32;2],
	fConst9: F32,
	fEntry18: F32,
	fRec68: [F32;2],
	fRec5: [F32;2],
	fRec4: [F32;3],
	fRec70: [F32;2],
	fRec69: [F32;3],
	fEntry19: F32,
	fRec71: [F32;2],
	fConst10: F32,
	fEntry20: F32,
	fRec72: [F32;2],
	fEntry21: F32,
	fRec73: [F32;2],
	fConst11: F32,
	fRec3: [F32;3],
	fVec18: [F32;2],
	fConst12: F32,
	fEntry22: F32,
	fRec74: [F32;2],
	fEntry23: F32,
	fRec75: [F32;2],
	fRec2: [F32;2],
	fRec1: [F32;3],
	fEntry24: F32,
	fRec76: [F32;2],
	fRec78: [F32;2],
	fRec77: [F32;3],
	fEntry25: F32,
	fRec79: [F32;2],
	fVec19: [F32;131072],
	fVec20: [F32;2],
	fRec84: [F32;2],
	fRec83: [F32;3],
	fRec86: [F32;2],
	fRec85: [F32;3],
	fRec82: [F32;3],
	fVec21: [F32;2],
	fRec81: [F32;2],
	fRec80: [F32;3],
	fRec88: [F32;2],
	fRec87: [F32;3],
	fEntry26: F32,
	fRec89: [F32;2],
	fVec22: [F32;2],
	fRec0: [F32;2],
	fEntry27: F32,
	fRec90: [F32;2],
	fVec23: [F32;2],
	fRec91: [F32;2],
	fEntry28: F32,
	fRec92: [F32;2],
	fEntry29: F32,
	fRec95: [F32;2],
	fButton2: F32,
	fRec96: [F32;2],
	fRec94: [F32;2],
	fRec93: [F32;2],
	fRec98: [F32;2],
	fRec97: [F32;2],
	fEntry30: F32,
	fRec99: [F32;2],
	fEntry31: F32,
	fRec100: [F32;2],
}

impl FaustDsp for insert {
	type T = F32;
		
	fn new() -> insert { 
		insert {
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fButton0: 0.0,
			fVec1: [0.0;2],
			iRec7: [0;2],
			fConst2: 0.0,
			fEntry0: 0.0,
			iVec2: [0;2],
			fEntry1: 0.0,
			fRec6: [0.0;2],
			fEntry2: 0.0,
			fButton1: 0.0,
			fEntry3: 0.0,
			fRec9: [0.0;2],
			fRec8: [0.0;2],
			IOTA: 0,
			fVec3: [0.0;131072],
			fRec10: [0.0;2],
			fEntry4: 0.0,
			fRec11: [0.0;2],
			fEntry5: 0.0,
			fRec12: [0.0;2],
			fEntry6: 0.0,
			fRec13: [0.0;2],
			fEntry7: 0.0,
			fRec14: [0.0;2],
			iRec15: [0;2],
			fRec16: [0.0;2],
			fEntry8: 0.0,
			fRec17: [0.0;2],
			fEntry9: 0.0,
			fRec18: [0.0;2],
			fConst3: 0.0,
			fRec19: [0.0;2],
			fConst4: 0.0,
			fEntry10: 0.0,
			fRec21: [0.0;2],
			fEntry11: 0.0,
			fRec22: [0.0;2],
			fRec20: [0.0;2],
			fVec4: [0.0;2],
			fVec5: [0.0;4096],
			fConst5: 0.0,
			iRec23: [0;2],
			fRec24: [0.0;2],
			fVec6: [0.0;2],
			fVec7: [0.0;4096],
			fRec25: [0.0;2],
			fRec26: [0.0;2],
			fConst6: 0.0,
			fRec28: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fRec29: [0.0;2],
			fRec30: [0.0;2],
			fRec32: [0.0;2],
			fRec33: [0.0;2],
			fRec34: [0.0;2],
			fEntry12: 0.0,
			fRec36: [0.0;2],
			fEntry13: 0.0,
			fRec37: [0.0;2],
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fEntry14: 0.0,
			fRec41: [0.0;2],
			fRec40: [0.0;2],
			fRec43: [0.0;2],
			fVec9: [0.0;2],
			fVec10: [0.0;4096],
			fRec42: [0.0;2],
			fRec44: [0.0;2],
			fRec46: [0.0;2],
			fRec48: [0.0;2],
			fVec11: [0.0;2],
			fVec12: [0.0;4096],
			fRec47: [0.0;2],
			fRec49: [0.0;2],
			fRec51: [0.0;2],
			fEntry15: 0.0,
			fRec52: [0.0;2],
			fEntry16: 0.0,
			fRec53: [0.0;2],
			fRec54: [0.0;2],
			fRec55: [0.0;2],
			fEntry17: 0.0,
			fRec57: [0.0;2],
			fRec56: [0.0;2],
			fRec59: [0.0;2],
			fVec13: [0.0;2],
			fVec14: [0.0;4096],
			fRec58: [0.0;2],
			fRec60: [0.0;2],
			fRec62: [0.0;2],
			fRec64: [0.0;2],
			fVec15: [0.0;2],
			fVec16: [0.0;4096],
			fRec63: [0.0;2],
			fRec65: [0.0;2],
			fRec67: [0.0;2],
			fVec17: [0.0;2],
			fConst9: 0.0,
			fEntry18: 0.0,
			fRec68: [0.0;2],
			fRec5: [0.0;2],
			fRec4: [0.0;3],
			fRec70: [0.0;2],
			fRec69: [0.0;3],
			fEntry19: 0.0,
			fRec71: [0.0;2],
			fConst10: 0.0,
			fEntry20: 0.0,
			fRec72: [0.0;2],
			fEntry21: 0.0,
			fRec73: [0.0;2],
			fConst11: 0.0,
			fRec3: [0.0;3],
			fVec18: [0.0;2],
			fConst12: 0.0,
			fEntry22: 0.0,
			fRec74: [0.0;2],
			fEntry23: 0.0,
			fRec75: [0.0;2],
			fRec2: [0.0;2],
			fRec1: [0.0;3],
			fEntry24: 0.0,
			fRec76: [0.0;2],
			fRec78: [0.0;2],
			fRec77: [0.0;3],
			fEntry25: 0.0,
			fRec79: [0.0;2],
			fVec19: [0.0;131072],
			fVec20: [0.0;2],
			fRec84: [0.0;2],
			fRec83: [0.0;3],
			fRec86: [0.0;2],
			fRec85: [0.0;3],
			fRec82: [0.0;3],
			fVec21: [0.0;2],
			fRec81: [0.0;2],
			fRec80: [0.0;3],
			fRec88: [0.0;2],
			fRec87: [0.0;3],
			fEntry26: 0.0,
			fRec89: [0.0;2],
			fVec22: [0.0;2],
			fRec0: [0.0;2],
			fEntry27: 0.0,
			fRec90: [0.0;2],
			fVec23: [0.0;2],
			fRec91: [0.0;2],
			fEntry28: 0.0,
			fRec92: [0.0;2],
			fEntry29: 0.0,
			fRec95: [0.0;2],
			fButton2: 0.0,
			fRec96: [0.0;2],
			fRec94: [0.0;2],
			fRec93: [0.0;2],
			fRec98: [0.0;2],
			fRec97: [0.0;2],
			fEntry30: 0.0,
			fRec99: [0.0;2],
			fEntry31: 0.0,
			fRec100: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/name", "Faust Analyzer Library");
		m.declare("analyzers.lib/version", "0.1");
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("envelopes.lib/adsr:author", "Yann Orlarey and Andrey Bundin");
		m.declare("envelopes.lib/ar:author", "Yann Orlarey, Stéphane Letz");
		m.declare("envelopes.lib/author", "GRAME");
		m.declare("envelopes.lib/copyright", "GRAME");
		m.declare("envelopes.lib/license", "LGPL with exception");
		m.declare("envelopes.lib/name", "Faust Envelope Library");
		m.declare("envelopes.lib/version", "0.1");
		m.declare("filename", "insert.dsp");
		m.declare("filters.lib/dcblocker:author", "Julius O. Smith III");
		m.declare("filters.lib/dcblocker:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/dcblocker:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/filterbank:author", "Julius O. Smith III");
		m.declare("filters.lib/filterbank:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/filterbank:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", "Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/highshelf:author", "Julius O. Smith III");
		m.declare("filters.lib/highshelf:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/highshelf:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/low_shelf:author", "Julius O. Smith III");
		m.declare("filters.lib/low_shelf:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/low_shelf:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass0_highpass1:author", "Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", "Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowshelf:author", "Julius O. Smith III");
		m.declare("filters.lib/lowshelf:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowshelf:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/peak_eq:author", "Julius O. Smith III");
		m.declare("filters.lib/peak_eq:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/peak_eq:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/peak_eq_cq:author", "Julius O. Smith III");
		m.declare("filters.lib/peak_eq_cq:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/peak_eq_cq:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/pole:author", "Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1:author", "Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", "Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/version", "0.3");
		m.declare("filters.lib/zero:author", "Julius O. Smith III");
		m.declare("filters.lib/zero:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/zero:license", "MIT-style STK-4.3 license");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.3");
		m.declare("misceffects.lib/name", "Misc Effects Library");
		m.declare("misceffects.lib/version", "2.0");
		m.declare("name", "insert");
		m.declare("noises.lib/name", "Faust Noise Generator Library");
		m.declare("noises.lib/version", "0.0");
		m.declare("oscillators.lib/name", "Faust Oscillator Library");
		m.declare("oscillators.lib/version", "0.1");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
		m.declare("routes.lib/name", "Faust Signal Routing Library");
		m.declare("routes.lib/version", "0.2");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.0");
		m.declare("spats.lib/name", "Faust Spatialization Library");
		m.declare("spats.lib/version", "0.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 4;
	}
	fn get_num_outputs(&self) -> i32 {
		return 8;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: insertSIG0 = newinsertSIG0();
		sig0.instance_initinsertSIG0(sample_rate);
		sig0.fillinsertSIG0(65536, unsafe { &mut ftbl0insertSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fButton0 = 0.0;
		self.fEntry0 = 0.0;
		self.fEntry1 = 25.0;
		self.fEntry2 = 0.0;
		self.fButton1 = 0.0;
		self.fEntry3 = 0.0;
		self.fEntry4 = 0.0;
		self.fEntry5 = 0.0;
		self.fEntry6 = 50.0;
		self.fEntry7 = 0.0;
		self.fEntry8 = 0.0;
		self.fEntry9 = 0.0;
		self.fEntry10 = 25.0;
		self.fEntry11 = 0.0;
		self.fEntry12 = 0.0;
		self.fEntry13 = 0.0;
		self.fEntry14 = 0.0;
		self.fEntry15 = 0.0;
		self.fEntry16 = 0.0;
		self.fEntry17 = 0.0;
		self.fEntry18 = 0.0;
		self.fEntry19 = 0.0;
		self.fEntry20 = 0.0;
		self.fEntry21 = 0.0;
		self.fEntry22 = 0.0;
		self.fEntry23 = 0.0;
		self.fEntry24 = 0.0;
		self.fEntry25 = 0.0;
		self.fEntry26 = 0.0;
		self.fEntry27 = 0.0;
		self.fEntry28 = 50.0;
		self.fEntry29 = 0.0;
		self.fButton2 = 0.0;
		self.fEntry30 = 0.0;
		self.fEntry31 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fVec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.iRec7[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.iVec2[l3 as usize] = 0;
		}
		for l4 in 0..2 {
			self.fRec6[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec9[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec8[l6 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l7 in 0..131072 {
			self.fVec3[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec10[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec11[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec12[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec13[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec14[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.iRec15[l13 as usize] = 0;
		}
		for l14 in 0..2 {
			self.fRec16[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec17[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec18[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec19[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec21[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec22[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec20[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fVec4[l21 as usize] = 0.0;
		}
		for l22 in 0..4096 {
			self.fVec5[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.iRec23[l23 as usize] = 0;
		}
		for l24 in 0..2 {
			self.fRec24[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec6[l25 as usize] = 0.0;
		}
		for l26 in 0..4096 {
			self.fVec7[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec25[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec26[l28 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec28[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec29[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec30[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec32[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec33[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec34[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec36[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec37[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec38[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec39[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec41[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec40[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec43[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fVec9[l44 as usize] = 0.0;
		}
		for l45 in 0..4096 {
			self.fVec10[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec42[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec44[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec46[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec48[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fVec11[l50 as usize] = 0.0;
		}
		for l51 in 0..4096 {
			self.fVec12[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec47[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec49[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec51[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec52[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec53[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec54[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec55[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec57[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec56[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec59[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fVec13[l62 as usize] = 0.0;
		}
		for l63 in 0..4096 {
			self.fVec14[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec58[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec60[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec62[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec64[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fVec15[l68 as usize] = 0.0;
		}
		for l69 in 0..4096 {
			self.fVec16[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec63[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec65[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec67[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fVec17[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec68[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec5[l75 as usize] = 0.0;
		}
		for l76 in 0..3 {
			self.fRec4[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec70[l77 as usize] = 0.0;
		}
		for l78 in 0..3 {
			self.fRec69[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec71[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec72[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec73[l81 as usize] = 0.0;
		}
		for l82 in 0..3 {
			self.fRec3[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fVec18[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec74[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec75[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec2[l86 as usize] = 0.0;
		}
		for l87 in 0..3 {
			self.fRec1[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec76[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec78[l89 as usize] = 0.0;
		}
		for l90 in 0..3 {
			self.fRec77[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec79[l91 as usize] = 0.0;
		}
		for l92 in 0..131072 {
			self.fVec19[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fVec20[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec84[l94 as usize] = 0.0;
		}
		for l95 in 0..3 {
			self.fRec83[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec86[l96 as usize] = 0.0;
		}
		for l97 in 0..3 {
			self.fRec85[l97 as usize] = 0.0;
		}
		for l98 in 0..3 {
			self.fRec82[l98 as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fVec21[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec81[l100 as usize] = 0.0;
		}
		for l101 in 0..3 {
			self.fRec80[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec88[l102 as usize] = 0.0;
		}
		for l103 in 0..3 {
			self.fRec87[l103 as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec89[l104 as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fVec22[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec0[l106 as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec90[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fVec23[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec91[l109 as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec92[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec95[l111 as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec96[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec94[l113 as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec93[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec98[l115 as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec97[l116 as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec99[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec100[l118 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32)));
		self.fConst1 = (1.0 - (44.0999985 / self.fConst0));
		self.fConst2 = (2.0 * self.fConst0);
		self.fConst3 = (0.125 * self.fConst0);
		self.fConst4 = (1.0 / self.fConst0);
		self.fConst5 = (0.5 * self.fConst0);
		self.fConst6 = (440.0 / self.fConst0);
		self.fConst7 = (1760.0 / self.fConst0);
		self.fConst8 = (0.25 * self.fConst0);
		self.fConst9 = (942.477783 / self.fConst0);
		self.fConst10 = (3141.59277 / self.fConst0);
		self.fConst11 = (6283.18555 / self.fConst0);
		self.fConst12 = (9424.77832 / self.fConst0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		insert::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("insert");
		ui_interface.add_num_entry("attack", ParamIndex(0), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("cutoff", ParamIndex(1), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("decay", ParamIndex(2), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("drive", ParamIndex(3), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("duckBy", ParamIndex(4), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_button("duckRelease", ParamIndex(5));
		ui_interface.add_num_entry("echo", ParamIndex(6), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_button("gate", ParamIndex(7));
		ui_interface.add_num_entry("highFreq", ParamIndex(8), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("highRes", ParamIndex(9), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("holdSample", ParamIndex(10), 0.0, 0.0, 1.0, 0.0);
		ui_interface.add_num_entry("lowFreq", ParamIndex(11), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("lowRes", ParamIndex(12), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("main", ParamIndex(13), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("midFreq", ParamIndex(14), 0.0, -25.0, 22.0, 10.0);
		ui_interface.add_num_entry("midRes", ParamIndex(15), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_button("note", ParamIndex(16));
		ui_interface.add_num_entry("pan", ParamIndex(17), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("release", ParamIndex(18), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("reverb", ParamIndex(19), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("sampleDetune", ParamIndex(20), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("sampleLevel", ParamIndex(21), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("sampleType", ParamIndex(22), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("spread", ParamIndex(23), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("sustain", ParamIndex(24), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth1Detune", ParamIndex(25), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth1Level", ParamIndex(26), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth1Type", ParamIndex(27), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("synth2Detune", ParamIndex(28), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth2Level", ParamIndex(29), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth2Type", ParamIndex(30), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("synth3Detune", ParamIndex(31), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth3Level", ParamIndex(32), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth3Type", ParamIndex(33), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("toDuck", ParamIndex(34), 0.0, 0.0, 50.0, 10.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			7 => Some(self.fButton0),
			16 => Some(self.fButton1),
			5 => Some(self.fButton2),
			18 => Some(self.fEntry0),
			21 => Some(self.fEntry1),
			23 => Some(self.fEntry10),
			25 => Some(self.fEntry11),
			29 => Some(self.fEntry12),
			30 => Some(self.fEntry13),
			28 => Some(self.fEntry14),
			32 => Some(self.fEntry15),
			33 => Some(self.fEntry16),
			31 => Some(self.fEntry17),
			11 => Some(self.fEntry18),
			12 => Some(self.fEntry19),
			22 => Some(self.fEntry2),
			14 => Some(self.fEntry20),
			15 => Some(self.fEntry21),
			8 => Some(self.fEntry22),
			1 => Some(self.fEntry23),
			9 => Some(self.fEntry24),
			17 => Some(self.fEntry25),
			3 => Some(self.fEntry26),
			34 => Some(self.fEntry27),
			13 => Some(self.fEntry28),
			4 => Some(self.fEntry29),
			20 => Some(self.fEntry3),
			6 => Some(self.fEntry30),
			19 => Some(self.fEntry31),
			10 => Some(self.fEntry4),
			0 => Some(self.fEntry5),
			24 => Some(self.fEntry6),
			2 => Some(self.fEntry7),
			26 => Some(self.fEntry8),
			27 => Some(self.fEntry9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			7 => { self.fButton0 = value }
			16 => { self.fButton1 = value }
			5 => { self.fButton2 = value }
			18 => { self.fEntry0 = value }
			21 => { self.fEntry1 = value }
			23 => { self.fEntry10 = value }
			25 => { self.fEntry11 = value }
			29 => { self.fEntry12 = value }
			30 => { self.fEntry13 = value }
			28 => { self.fEntry14 = value }
			32 => { self.fEntry15 = value }
			33 => { self.fEntry16 = value }
			31 => { self.fEntry17 = value }
			11 => { self.fEntry18 = value }
			12 => { self.fEntry19 = value }
			22 => { self.fEntry2 = value }
			14 => { self.fEntry20 = value }
			15 => { self.fEntry21 = value }
			8 => { self.fEntry22 = value }
			1 => { self.fEntry23 = value }
			9 => { self.fEntry24 = value }
			17 => { self.fEntry25 = value }
			3 => { self.fEntry26 = value }
			34 => { self.fEntry27 = value }
			13 => { self.fEntry28 = value }
			4 => { self.fEntry29 = value }
			20 => { self.fEntry3 = value }
			6 => { self.fEntry30 = value }
			19 => { self.fEntry31 = value }
			10 => { self.fEntry4 = value }
			0 => { self.fEntry5 = value }
			24 => { self.fEntry6 = value }
			2 => { self.fEntry7 = value }
			26 => { self.fEntry8 = value }
			27 => { self.fEntry9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2, inputs3) = if let [inputs0, inputs1, inputs2, inputs3, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			let inputs3 = inputs3[..count as usize].iter();
			(inputs0, inputs1, inputs2, inputs3)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7) = if let [outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			let outputs2 = outputs2[..count as usize].iter_mut();
			let outputs3 = outputs3[..count as usize].iter_mut();
			let outputs4 = outputs4[..count as usize].iter_mut();
			let outputs5 = outputs5[..count as usize].iter_mut();
			let outputs6 = outputs6[..count as usize].iter_mut();
			let outputs7 = outputs7[..count as usize].iter_mut();
			(outputs0, outputs1, outputs2, outputs3, outputs4, outputs5, outputs6, outputs7)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = (self.fButton0 as F32);
		let mut fSlow1: F32 = (self.fEntry0 as F32);
		let mut fSlow2: F32 = (1.0 / F32::max(1.0, (self.fConst2 * insert_faustpower2_f((0.0199999996 * fSlow1)))));
		let mut fSlow3: F32 = (self.fEntry1 as F32);
		let mut fSlow4: F32 = (self.fEntry2 as F32);
		let mut fSlow5: F32 = (((((fSlow4 == ((1 | (fSlow4 as i32)) as F32)) as i32) == 2) as i32) as F32);
		let mut fSlow6: F32 = (self.fButton1 as F32);
		let mut fSlow7: F32 = (self.fEntry3 as F32);
		let mut fSlow8: F32 = (0.0833333358 * fSlow5);
		let mut iSlow9: i32 = ((self.fEntry4 as F32) as i32);
		let mut fSlow10: F32 = (self.fEntry5 as F32);
		let mut fSlow11: F32 = (self.fEntry6 as F32);
		let mut fSlow12: F32 = (self.fEntry7 as F32);
		let mut iSlow13: i32 = ((fSlow0 == 0.0) as i32);
		let mut fSlow14: F32 = (self.fEntry8 as F32);
		let mut fSlow15: F32 = (self.fEntry9 as F32);
		let mut fSlow16: F32 = (((fSlow15 >= 3.0) as i32) as F32);
		let mut fSlow17: F32 = (((fSlow15 >= 4.0) as i32) as F32);
		let mut fSlow18: F32 = (self.fEntry10 as F32);
		let mut fSlow19: F32 = (self.fEntry11 as F32);
		let mut fSlow20: F32 = (((fSlow15 >= 2.0) as i32) as F32);
		let mut fSlow21: F32 = (((fSlow15 >= 1.0) as i32) as F32);
		let mut fSlow22: F32 = (self.fEntry12 as F32);
		let mut fSlow23: F32 = (self.fEntry13 as F32);
		let mut fSlow24: F32 = (((fSlow23 >= 3.0) as i32) as F32);
		let mut fSlow25: F32 = (((fSlow23 >= 2.0) as i32) as F32);
		let mut fSlow26: F32 = (((fSlow23 >= 1.0) as i32) as F32);
		let mut fSlow27: F32 = (self.fEntry14 as F32);
		let mut fSlow28: F32 = (((fSlow23 >= 4.0) as i32) as F32);
		let mut fSlow29: F32 = (self.fEntry15 as F32);
		let mut fSlow30: F32 = (self.fEntry16 as F32);
		let mut fSlow31: F32 = (((fSlow30 >= 3.0) as i32) as F32);
		let mut fSlow32: F32 = (((fSlow30 >= 2.0) as i32) as F32);
		let mut fSlow33: F32 = (((fSlow30 >= 1.0) as i32) as F32);
		let mut fSlow34: F32 = (self.fEntry17 as F32);
		let mut fSlow35: F32 = (((fSlow30 >= 4.0) as i32) as F32);
		let mut fSlow36: F32 = (self.fEntry18 as F32);
		let mut fSlow37: F32 = (self.fEntry19 as F32);
		let mut fSlow38: F32 = (self.fEntry20 as F32);
		let mut fSlow39: F32 = (self.fEntry21 as F32);
		let mut fSlow40: F32 = (self.fEntry22 as F32);
		let mut fSlow41: F32 = (self.fEntry23 as F32);
		let mut fSlow42: F32 = (self.fEntry24 as F32);
		let mut fSlow43: F32 = (self.fEntry25 as F32);
		let mut fSlow44: F32 = (self.fEntry26 as F32);
		let mut fSlow45: F32 = (self.fEntry27 as F32);
		let mut fSlow46: F32 = (self.fEntry28 as F32);
		let mut fSlow47: F32 = (self.fEntry29 as F32);
		let mut fSlow48: F32 = (self.fButton2 as F32);
		let mut fSlow49: F32 = (self.fEntry30 as F32);
		let mut fSlow50: F32 = (self.fEntry31 as F32);
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7);
		for (((((((((((input0, input1), input2), input3), output0), output1), output2), output3), output4), output5), output6), output7) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow0;
			self.iRec7[0] = (((fSlow0 > self.fVec1[1]) as i32) + (((fSlow0 <= self.fVec1[1]) as i32) * (self.iRec7[1] + ((self.iRec7[1] > 0) as i32))));
			let mut fTemp0: F32 = (self.iRec7[0] as F32);
			let mut fTemp1: F32 = F32::max(0.0, F32::min(fTemp0, ((fSlow2 * (1.0 - fTemp0)) + 1.0)));
			let mut iTemp2: i32 = (((fTemp1 > 0.0) as i32) - ((fTemp1 < 0.0) as i32));
			self.iVec2[0] = iTemp2;
			let mut fTemp3: F32 = ((((iTemp2 == self.iVec2[1]) as i32) | ((iTemp2 == 0) as i32)) as F32);
			let mut fTemp4: F32 = (1.0 - (self.fConst1 * fTemp3));
			self.fRec6[0] = ((self.fConst1 * (fTemp3 * self.fRec6[1])) + (fSlow3 * fTemp4));
			let mut fTemp5: F32 = (*input2 as F32);
			let mut fTemp6: F32 = (self.fRec8[1] + 1.0);
			self.fRec9[0] = ((self.fConst1 * (fTemp3 * self.fRec9[1])) + (fSlow7 * fTemp4));
			let mut fTemp7: F32 = (fSlow6 + ((0.100000001 * self.fRec9[0]) + -69.0));
			let mut fTemp8: F32 = ((((fSlow5 * fTemp7) == 0.0) as i32) as F32);
			let mut fTemp9: F32 = (self.fRec8[1] + -1.0);
			self.fRec8[0] = if (((fTemp6 < fTemp8) as i32) as i32 != 0) { fTemp6 } else { if (((fTemp9 > fTemp8) as i32) as i32 != 0) { fTemp9 } else { fTemp8 } };
			let mut fTemp10: F32 = (1.0 - self.fRec8[0]);
			let mut fTemp11: F32 = (fTemp5 * fTemp10);
			self.fVec3[(self.IOTA & 131071) as usize] = fTemp11;
			self.fRec10[0] = libm::fmodf((self.fRec10[1] + (1001.0 - F32::powf(2.0, (fSlow8 * fTemp7)))), 1000.0);
			let mut iTemp12: i32 = (self.fRec10[0] as i32);
			let mut iTemp13: i32 = std::cmp::min(65537, (std::cmp::max(0, (iTemp12 as i32)) as i32));
			let mut fTemp14: F32 = F32::floor(self.fRec10[0]);
			let mut fTemp15: F32 = (fTemp14 + (1.0 - self.fRec10[0]));
			let mut fTemp16: F32 = (self.fRec10[0] - fTemp14);
			let mut iTemp17: i32 = std::cmp::min(65537, (std::cmp::max(0, ((iTemp12 + 1) as i32)) as i32));
			let mut fTemp18: F32 = F32::min((0.100000001 * self.fRec10[0]), 1.0);
			let mut fTemp19: F32 = (self.fRec10[0] + 1000.0);
			let mut iTemp20: i32 = (fTemp19 as i32);
			let mut iTemp21: i32 = std::cmp::min(65537, (std::cmp::max(0, (iTemp20 as i32)) as i32));
			let mut fTemp22: F32 = F32::floor(fTemp19);
			let mut fTemp23: F32 = (fTemp22 + (-999.0 - self.fRec10[0]));
			let mut iTemp24: i32 = std::cmp::min(65537, (std::cmp::max(0, ((iTemp20 + 1) as i32)) as i32));
			let mut fTemp25: F32 = (self.fRec10[0] + (1000.0 - fTemp22));
			let mut fTemp26: F32 = (1.0 - fTemp18);
			self.fRec11[0] = (fSlow0 + (self.fRec11[1] * (((self.fVec1[1] >= fSlow0) as i32) as F32)));
			self.fRec12[0] = ((self.fConst1 * (fTemp3 * self.fRec12[1])) + (fSlow10 * fTemp4));
			let mut fTemp27: F32 = F32::max(1.0, (self.fConst2 * insert_faustpower2_f((0.0199999996 * self.fRec12[0]))));
			self.fRec13[0] = ((self.fConst1 * (fTemp3 * self.fRec13[1])) + (fSlow11 * fTemp4));
			let mut fTemp28: F32 = (0.0199999996 * self.fRec13[0]);
			self.fRec14[0] = ((self.fConst1 * (fTemp3 * self.fRec14[1])) + (fSlow12 * fTemp4));
			self.iRec15[0] = (iSlow13 * (self.iRec15[1] + 1));
			self.fRec16[0] = ((self.fConst1 * (fTemp3 * self.fRec16[1])) + (fSlow1 * fTemp4));
			let mut fTemp29: F32 = F32::max(0.0, (F32::min((self.fRec11[0] / fTemp27), F32::max(((((1.0 - fTemp28) * (fTemp27 - self.fRec11[0])) / F32::max(1.0, (self.fConst2 * insert_faustpower2_f((0.0199999996 * self.fRec14[0]))))) + 1.0), fTemp28)) * (1.0 - ((self.iRec15[0] as F32) / F32::max(1.0, (self.fConst2 * insert_faustpower2_f((0.0199999996 * self.fRec16[0]))))))));
			let mut fTemp30: F32 = if (iSlow9 as i32 != 0) { fTemp29 } else { 1.0 };
			self.fRec17[0] = ((self.fConst1 * (fTemp3 * self.fRec17[1])) + (fSlow14 * fTemp4));
			let mut fTemp31: F32 = (self.fRec18[1] + 1.0);
			let mut fTemp32: F32 = (self.fRec18[1] + -1.0);
			self.fRec18[0] = if (((fTemp31 < fSlow16) as i32) as i32 != 0) { fTemp31 } else { if (((fTemp32 > fSlow16) as i32) as i32 != 0) { fTemp32 } else { fSlow16 } };
			let mut fTemp33: F32 = (self.fRec19[1] + 1.0);
			let mut fTemp34: F32 = (self.fRec19[1] + -1.0);
			self.fRec19[0] = if (((fTemp33 < fSlow17) as i32) as i32 != 0) { fTemp33 } else { if (((fTemp34 > fSlow17) as i32) as i32 != 0) { fTemp34 } else { fSlow17 } };
			let mut fTemp35: F32 = (1.0 - self.fRec19[0]);
			let mut fTemp36: F32 = (self.iVec0[1] as F32);
			self.fRec21[0] = ((self.fConst1 * (fTemp3 * self.fRec21[1])) + (fSlow18 * fTemp4));
			let mut fTemp37: F32 = (0.00200000009 * (0.0 - self.fRec21[0]));
			self.fRec22[0] = ((self.fConst1 * (fTemp3 * self.fRec22[1])) + (fSlow19 * fTemp4));
			let mut fTemp38: F32 = (0.00999999978 * self.fRec22[0]);
			let mut fTemp39: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((fTemp37 + fTemp38) + -69.0))));
			let mut fTemp40: F32 = (440.0 * fTemp39);
			let mut fTemp41: F32 = F32::max(fTemp40, 23.4489498);
			let mut fTemp42: F32 = F32::max(20.0, F32::abs(fTemp41));
			let mut fTemp43: F32 = (self.fRec20[1] + (self.fConst4 * fTemp42));
			self.fRec20[0] = (fTemp43 - F32::floor(fTemp43));
			let mut fTemp44: F32 = insert_faustpower2_f(((2.0 * self.fRec20[0]) + -1.0));
			self.fVec4[0] = fTemp44;
			let mut fTemp45: F32 = ((fTemp36 * (fTemp44 - self.fVec4[1])) / fTemp42);
			self.fVec5[(self.IOTA & 4095) as usize] = fTemp45;
			let mut fTemp46: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst5 / fTemp41)));
			let mut iTemp47: i32 = (fTemp46 as i32);
			let mut fTemp48: F32 = F32::floor(fTemp46);
			let mut fTemp49: F32 = ((fTemp45 - (self.fVec5[((self.IOTA - iTemp47) & 4095) as usize] * (fTemp48 + (1.0 - fTemp46)))) - ((fTemp46 - fTemp48) * self.fVec5[((self.IOTA - (iTemp47 + 1)) & 4095) as usize]));
			self.iRec23[0] = ((1103515245 * self.iRec23[1]) + 12345);
			let mut fTemp50: F32 = (self.iRec23[0] as F32);
			let mut fTemp51: F32 = (1.16415322e-10 * (self.fRec19[0] * fTemp50));
			let mut fTemp52: F32 = (0.00200000009 * self.fRec21[0]);
			let mut fTemp53: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((fTemp38 + fTemp52) + -69.0))));
			let mut fTemp54: F32 = (440.0 * fTemp53);
			let mut fTemp55: F32 = F32::max(fTemp54, 23.4489498);
			let mut fTemp56: F32 = F32::max(20.0, F32::abs(fTemp55));
			let mut fTemp57: F32 = (self.fRec24[1] + (self.fConst4 * fTemp56));
			self.fRec24[0] = (fTemp57 - F32::floor(fTemp57));
			let mut fTemp58: F32 = insert_faustpower2_f(((2.0 * self.fRec24[0]) + -1.0));
			self.fVec6[0] = fTemp58;
			let mut fTemp59: F32 = ((fTemp36 * (fTemp58 - self.fVec6[1])) / fTemp56);
			self.fVec7[(self.IOTA & 4095) as usize] = fTemp59;
			let mut fTemp60: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst5 / fTemp55)));
			let mut iTemp61: i32 = (fTemp60 as i32);
			let mut fTemp62: F32 = F32::floor(fTemp60);
			let mut fTemp63: F32 = ((fTemp59 - (self.fVec7[((self.IOTA - iTemp61) & 4095) as usize] * (fTemp62 + (1.0 - fTemp60)))) - ((fTemp60 - fTemp62) * self.fVec7[((self.IOTA - (iTemp61 + 1)) & 4095) as usize]));
			let mut fTemp64: F32 = (self.fRec25[1] + 1.0);
			let mut fTemp65: F32 = (self.fRec25[1] + -1.0);
			self.fRec25[0] = if (((fTemp64 < fSlow20) as i32) as i32 != 0) { fTemp64 } else { if (((fTemp65 > fSlow20) as i32) as i32 != 0) { fTemp65 } else { fSlow20 } };
			let mut fTemp66: F32 = (1.0 - self.fRec25[0]);
			let mut fTemp67: F32 = (self.fRec26[1] + 1.0);
			let mut fTemp68: F32 = (self.fRec26[1] + -1.0);
			self.fRec26[0] = if (((fTemp67 < fSlow21) as i32) as i32 != 0) { fTemp67 } else { if (((fTemp68 > fSlow21) as i32) as i32 != 0) { fTemp68 } else { fSlow21 } };
			let mut fTemp69: F32 = (1.0 - self.fRec26[0]);
			let mut fTemp70: F32 = (self.fRec28[1] + (self.fConst6 * fTemp39));
			self.fRec28[0] = (fTemp70 - F32::floor(fTemp70));
			self.fRec29[0] = ((0.999000013 * self.fRec29[1]) + (self.fConst8 * fTemp49));
			let mut fTemp71: F32 = F32::max(1.1920929e-07, F32::abs(fTemp40));
			let mut fTemp72: F32 = (self.fRec30[1] + (self.fConst4 * fTemp71));
			let mut fTemp73: F32 = (fTemp72 + -1.0);
			let mut iTemp74: i32 = ((fTemp73 < 0.0) as i32);
			self.fRec30[0] = if (iTemp74 as i32 != 0) { fTemp72 } else { fTemp73 };
			let mut fRec31: F32 = if (iTemp74 as i32 != 0) { fTemp72 } else { (fTemp72 + (fTemp73 * (1.0 - (self.fConst0 / fTemp71)))) };
			let mut fTemp75: F32 = (self.fRec32[1] + (self.fConst6 * fTemp53));
			self.fRec32[0] = (fTemp75 - F32::floor(fTemp75));
			self.fRec33[0] = ((0.999000013 * self.fRec33[1]) + (self.fConst8 * fTemp63));
			let mut fTemp76: F32 = F32::max(1.1920929e-07, F32::abs(fTemp54));
			let mut fTemp77: F32 = (self.fRec34[1] + (self.fConst4 * fTemp76));
			let mut fTemp78: F32 = (fTemp77 + -1.0);
			let mut iTemp79: i32 = ((fTemp78 < 0.0) as i32);
			self.fRec34[0] = if (iTemp79 as i32 != 0) { fTemp77 } else { fTemp78 };
			let mut fRec35: F32 = if (iTemp79 as i32 != 0) { fTemp77 } else { (fTemp77 + (fTemp78 * (1.0 - (self.fConst0 / fTemp76)))) };
			self.fRec36[0] = ((self.fConst1 * (fTemp3 * self.fRec36[1])) + (fSlow22 * fTemp4));
			let mut fTemp80: F32 = (self.fRec37[1] + 1.0);
			let mut fTemp81: F32 = (self.fRec37[1] + -1.0);
			self.fRec37[0] = if (((fTemp80 < fSlow24) as i32) as i32 != 0) { fTemp80 } else { if (((fTemp81 > fSlow24) as i32) as i32 != 0) { fTemp81 } else { fSlow24 } };
			let mut fTemp82: F32 = (self.fRec38[1] + 1.0);
			let mut fTemp83: F32 = (self.fRec38[1] + -1.0);
			self.fRec38[0] = if (((fTemp82 < fSlow25) as i32) as i32 != 0) { fTemp82 } else { if (((fTemp83 > fSlow25) as i32) as i32 != 0) { fTemp83 } else { fSlow25 } };
			let mut fTemp84: F32 = (1.0 - self.fRec38[0]);
			let mut fTemp85: F32 = (self.fRec39[1] + 1.0);
			let mut fTemp86: F32 = (self.fRec39[1] + -1.0);
			self.fRec39[0] = if (((fTemp85 < fSlow26) as i32) as i32 != 0) { fTemp85 } else { if (((fTemp86 > fSlow26) as i32) as i32 != 0) { fTemp86 } else { fSlow26 } };
			let mut fTemp87: F32 = (1.0 - self.fRec39[0]);
			self.fRec41[0] = ((self.fConst1 * (fTemp3 * self.fRec41[1])) + (fSlow27 * fTemp4));
			let mut fTemp88: F32 = (0.00999999978 * self.fRec41[0]);
			let mut fTemp89: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((fTemp37 + fTemp88) + -69.0))));
			let mut fTemp90: F32 = (self.fRec40[1] + (self.fConst6 * fTemp89));
			self.fRec40[0] = (fTemp90 - F32::floor(fTemp90));
			let mut fTemp91: F32 = (440.0 * fTemp89);
			let mut fTemp92: F32 = F32::max(fTemp91, 23.4489498);
			let mut fTemp93: F32 = F32::max(20.0, F32::abs(fTemp92));
			let mut fTemp94: F32 = (self.fRec43[1] + (self.fConst4 * fTemp93));
			self.fRec43[0] = (fTemp94 - F32::floor(fTemp94));
			let mut fTemp95: F32 = insert_faustpower2_f(((2.0 * self.fRec43[0]) + -1.0));
			self.fVec9[0] = fTemp95;
			let mut fTemp96: F32 = ((fTemp36 * (fTemp95 - self.fVec9[1])) / fTemp93);
			self.fVec10[(self.IOTA & 4095) as usize] = fTemp96;
			let mut fTemp97: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst5 / fTemp92)));
			let mut iTemp98: i32 = (fTemp97 as i32);
			let mut fTemp99: F32 = F32::floor(fTemp97);
			let mut fTemp100: F32 = ((fTemp96 - (self.fVec10[((self.IOTA - iTemp98) & 4095) as usize] * (fTemp99 + (1.0 - fTemp97)))) - ((fTemp97 - fTemp99) * self.fVec10[((self.IOTA - (iTemp98 + 1)) & 4095) as usize]));
			self.fRec42[0] = ((0.999000013 * self.fRec42[1]) + (self.fConst8 * fTemp100));
			let mut fTemp101: F32 = F32::max(1.1920929e-07, F32::abs(fTemp91));
			let mut fTemp102: F32 = (self.fRec44[1] + (self.fConst4 * fTemp101));
			let mut fTemp103: F32 = (fTemp102 + -1.0);
			let mut iTemp104: i32 = ((fTemp103 < 0.0) as i32);
			self.fRec44[0] = if (iTemp104 as i32 != 0) { fTemp102 } else { fTemp103 };
			let mut fRec45: F32 = if (iTemp104 as i32 != 0) { fTemp102 } else { (fTemp102 + (fTemp103 * (1.0 - (self.fConst0 / fTemp101)))) };
			let mut fTemp105: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((fTemp52 + fTemp88) + -69.0))));
			let mut fTemp106: F32 = (self.fRec46[1] + (self.fConst6 * fTemp105));
			self.fRec46[0] = (fTemp106 - F32::floor(fTemp106));
			let mut fTemp107: F32 = (440.0 * fTemp105);
			let mut fTemp108: F32 = F32::max(fTemp107, 23.4489498);
			let mut fTemp109: F32 = F32::max(20.0, F32::abs(fTemp108));
			let mut fTemp110: F32 = (self.fRec48[1] + (self.fConst4 * fTemp109));
			self.fRec48[0] = (fTemp110 - F32::floor(fTemp110));
			let mut fTemp111: F32 = insert_faustpower2_f(((2.0 * self.fRec48[0]) + -1.0));
			self.fVec11[0] = fTemp111;
			let mut fTemp112: F32 = ((fTemp36 * (fTemp111 - self.fVec11[1])) / fTemp109);
			self.fVec12[(self.IOTA & 4095) as usize] = fTemp112;
			let mut fTemp113: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst5 / fTemp108)));
			let mut iTemp114: i32 = (fTemp113 as i32);
			let mut fTemp115: F32 = F32::floor(fTemp113);
			let mut fTemp116: F32 = ((fTemp112 - (self.fVec12[((self.IOTA - iTemp114) & 4095) as usize] * (fTemp115 + (1.0 - fTemp113)))) - ((fTemp113 - fTemp115) * self.fVec12[((self.IOTA - (iTemp114 + 1)) & 4095) as usize]));
			self.fRec47[0] = ((0.999000013 * self.fRec47[1]) + (self.fConst8 * fTemp116));
			let mut fTemp117: F32 = F32::max(1.1920929e-07, F32::abs(fTemp107));
			let mut fTemp118: F32 = (self.fRec49[1] + (self.fConst4 * fTemp117));
			let mut fTemp119: F32 = (fTemp118 + -1.0);
			let mut iTemp120: i32 = ((fTemp119 < 0.0) as i32);
			self.fRec49[0] = if (iTemp120 as i32 != 0) { fTemp118 } else { fTemp119 };
			let mut fRec50: F32 = if (iTemp120 as i32 != 0) { fTemp118 } else { (fTemp118 + (fTemp119 * (1.0 - (self.fConst0 / fTemp117)))) };
			let mut fTemp121: F32 = (self.fRec51[1] + 1.0);
			let mut fTemp122: F32 = (self.fRec51[1] + -1.0);
			self.fRec51[0] = if (((fTemp121 < fSlow28) as i32) as i32 != 0) { fTemp121 } else { if (((fTemp122 > fSlow28) as i32) as i32 != 0) { fTemp122 } else { fSlow28 } };
			let mut fTemp123: F32 = (1.0 - self.fRec51[0]);
			let mut fTemp124: F32 = (1.16415322e-10 * (self.fRec51[0] * fTemp50));
			self.fRec52[0] = ((self.fConst1 * (fTemp3 * self.fRec52[1])) + (fSlow29 * fTemp4));
			let mut fTemp125: F32 = (self.fRec53[1] + 1.0);
			let mut fTemp126: F32 = (self.fRec53[1] + -1.0);
			self.fRec53[0] = if (((fTemp125 < fSlow31) as i32) as i32 != 0) { fTemp125 } else { if (((fTemp126 > fSlow31) as i32) as i32 != 0) { fTemp126 } else { fSlow31 } };
			let mut fTemp127: F32 = (self.fRec54[1] + 1.0);
			let mut fTemp128: F32 = (self.fRec54[1] + -1.0);
			self.fRec54[0] = if (((fTemp127 < fSlow32) as i32) as i32 != 0) { fTemp127 } else { if (((fTemp128 > fSlow32) as i32) as i32 != 0) { fTemp128 } else { fSlow32 } };
			let mut fTemp129: F32 = (1.0 - self.fRec54[0]);
			let mut fTemp130: F32 = (self.fRec55[1] + 1.0);
			let mut fTemp131: F32 = (self.fRec55[1] + -1.0);
			self.fRec55[0] = if (((fTemp130 < fSlow33) as i32) as i32 != 0) { fTemp130 } else { if (((fTemp131 > fSlow33) as i32) as i32 != 0) { fTemp131 } else { fSlow33 } };
			let mut fTemp132: F32 = (1.0 - self.fRec55[0]);
			self.fRec57[0] = ((self.fConst1 * (fTemp3 * self.fRec57[1])) + (fSlow34 * fTemp4));
			let mut fTemp133: F32 = (0.00999999978 * self.fRec57[0]);
			let mut fTemp134: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((fTemp37 + fTemp133) + -69.0))));
			let mut fTemp135: F32 = (self.fRec56[1] + (self.fConst6 * fTemp134));
			self.fRec56[0] = (fTemp135 - F32::floor(fTemp135));
			let mut fTemp136: F32 = (440.0 * fTemp134);
			let mut fTemp137: F32 = F32::max(fTemp136, 23.4489498);
			let mut fTemp138: F32 = F32::max(20.0, F32::abs(fTemp137));
			let mut fTemp139: F32 = (self.fRec59[1] + (self.fConst4 * fTemp138));
			self.fRec59[0] = (fTemp139 - F32::floor(fTemp139));
			let mut fTemp140: F32 = insert_faustpower2_f(((2.0 * self.fRec59[0]) + -1.0));
			self.fVec13[0] = fTemp140;
			let mut fTemp141: F32 = ((fTemp36 * (fTemp140 - self.fVec13[1])) / fTemp138);
			self.fVec14[(self.IOTA & 4095) as usize] = fTemp141;
			let mut fTemp142: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst5 / fTemp137)));
			let mut iTemp143: i32 = (fTemp142 as i32);
			let mut fTemp144: F32 = F32::floor(fTemp142);
			let mut fTemp145: F32 = ((fTemp141 - (self.fVec14[((self.IOTA - iTemp143) & 4095) as usize] * (fTemp144 + (1.0 - fTemp142)))) - ((fTemp142 - fTemp144) * self.fVec14[((self.IOTA - (iTemp143 + 1)) & 4095) as usize]));
			self.fRec58[0] = ((0.999000013 * self.fRec58[1]) + (self.fConst8 * fTemp145));
			let mut fTemp146: F32 = F32::max(1.1920929e-07, F32::abs(fTemp136));
			let mut fTemp147: F32 = (self.fRec60[1] + (self.fConst4 * fTemp146));
			let mut fTemp148: F32 = (fTemp147 + -1.0);
			let mut iTemp149: i32 = ((fTemp148 < 0.0) as i32);
			self.fRec60[0] = if (iTemp149 as i32 != 0) { fTemp147 } else { fTemp148 };
			let mut fRec61: F32 = if (iTemp149 as i32 != 0) { fTemp147 } else { (fTemp147 + (fTemp148 * (1.0 - (self.fConst0 / fTemp146)))) };
			let mut fTemp150: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((fTemp52 + fTemp133) + -69.0))));
			let mut fTemp151: F32 = (self.fRec62[1] + (self.fConst6 * fTemp150));
			self.fRec62[0] = (fTemp151 - F32::floor(fTemp151));
			let mut fTemp152: F32 = (440.0 * fTemp150);
			let mut fTemp153: F32 = F32::max(fTemp152, 23.4489498);
			let mut fTemp154: F32 = F32::max(20.0, F32::abs(fTemp153));
			let mut fTemp155: F32 = (self.fRec64[1] + (self.fConst4 * fTemp154));
			self.fRec64[0] = (fTemp155 - F32::floor(fTemp155));
			let mut fTemp156: F32 = insert_faustpower2_f(((2.0 * self.fRec64[0]) + -1.0));
			self.fVec15[0] = fTemp156;
			let mut fTemp157: F32 = ((fTemp36 * (fTemp156 - self.fVec15[1])) / fTemp154);
			self.fVec16[(self.IOTA & 4095) as usize] = fTemp157;
			let mut fTemp158: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst5 / fTemp153)));
			let mut iTemp159: i32 = (fTemp158 as i32);
			let mut fTemp160: F32 = F32::floor(fTemp158);
			let mut fTemp161: F32 = ((fTemp157 - (self.fVec16[((self.IOTA - iTemp159) & 4095) as usize] * (fTemp160 + (1.0 - fTemp158)))) - ((fTemp158 - fTemp160) * self.fVec16[((self.IOTA - (iTemp159 + 1)) & 4095) as usize]));
			self.fRec63[0] = ((0.999000013 * self.fRec63[1]) + (self.fConst8 * fTemp161));
			let mut fTemp162: F32 = F32::max(1.1920929e-07, F32::abs(fTemp152));
			let mut fTemp163: F32 = (self.fConst4 * fTemp162);
			let mut fTemp164: F32 = (self.fRec65[1] + fTemp163);
			let mut fTemp165: F32 = (fTemp164 + -1.0);
			let mut iTemp166: i32 = ((fTemp165 < 0.0) as i32);
			self.fRec65[0] = if (iTemp166 as i32 != 0) { fTemp164 } else { fTemp165 };
			let mut fRec66: F32 = if (iTemp166 as i32 != 0) { fTemp164 } else { (fTemp163 + (self.fRec65[1] + (fTemp165 * (1.0 - (self.fConst0 / fTemp162))))) };
			let mut fTemp167: F32 = (self.fRec67[1] + 1.0);
			let mut fTemp168: F32 = (self.fRec67[1] + -1.0);
			self.fRec67[0] = if (((fTemp167 < fSlow35) as i32) as i32 != 0) { fTemp167 } else { if (((fTemp168 > fSlow35) as i32) as i32 != 0) { fTemp168 } else { fSlow35 } };
			let mut fTemp169: F32 = (1.0 - self.fRec67[0]);
			let mut fTemp170: F32 = (1.16415322e-10 * (self.fRec67[0] * fTemp50));
			let mut fTemp171: F32 = (0.00222222228 * (fTemp29 * (((self.fRec17[0] * ((self.fRec18[0] * (((self.fConst3 * (fTemp35 * fTemp49)) + fTemp51) + (fTemp51 + (self.fConst3 * (fTemp35 * fTemp63))))) + ((1.0 - self.fRec18[0]) * (((fTemp66 * ((fTemp69 * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec28[0]) as i32) as usize] }) + (self.fConst7 * ((self.fRec26[0] * self.fRec29[0]) * fTemp39)))) + (0.5 * (self.fRec25[0] * ((2.0 * fRec31) + -1.0)))) + ((fTemp66 * ((fTemp69 * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec32[0]) as i32) as usize] }) + (self.fConst7 * ((self.fRec26[0] * self.fRec33[0]) * fTemp53)))) + (0.5 * (self.fRec25[0] * ((2.0 * fRec35) + -1.0)))))))) + (self.fRec36[0] * (((1.0 - self.fRec37[0]) * (((fTemp84 * ((fTemp87 * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec40[0]) as i32) as usize] }) + (self.fConst7 * ((self.fRec39[0] * self.fRec42[0]) * fTemp89)))) + (0.5 * (self.fRec38[0] * ((2.0 * fRec45) + -1.0)))) + ((fTemp84 * ((fTemp87 * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec46[0]) as i32) as usize] }) + (self.fConst7 * ((self.fRec39[0] * self.fRec47[0]) * fTemp105)))) + (0.5 * (self.fRec38[0] * ((2.0 * fRec50) + -1.0)))))) + (self.fRec37[0] * (((self.fConst3 * (fTemp123 * fTemp100)) + fTemp124) + (fTemp124 + (self.fConst3 * (fTemp123 * fTemp116)))))))) + (self.fRec52[0] * (((1.0 - self.fRec53[0]) * (((fTemp129 * ((fTemp132 * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec56[0]) as i32) as usize] }) + (self.fConst7 * ((self.fRec55[0] * self.fRec58[0]) * fTemp134)))) + (0.5 * (self.fRec54[0] * ((2.0 * fRec61) + -1.0)))) + ((fTemp129 * ((fTemp132 * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec62[0]) as i32) as usize] }) + (self.fConst7 * ((self.fRec55[0] * self.fRec63[0]) * fTemp150)))) + (0.5 * (self.fRec54[0] * ((2.0 * fRec66) + -1.0)))))) + (self.fRec53[0] * (((self.fConst3 * (fTemp169 * fTemp145)) + fTemp170) + (fTemp170 + (self.fConst3 * (fTemp169 * fTemp161))))))))));
			let mut fTemp172: F32 = ((0.0399999991 * ((self.fRec6[0] * ((fTemp5 * self.fRec8[0]) + (fTemp10 * ((((self.fVec3[((self.IOTA - iTemp13) & 131071) as usize] * fTemp15) + (fTemp16 * self.fVec3[((self.IOTA - iTemp17) & 131071) as usize])) * fTemp18) + (((self.fVec3[((self.IOTA - iTemp21) & 131071) as usize] * fTemp23) + (self.fVec3[((self.IOTA - iTemp24) & 131071) as usize] * fTemp25)) * fTemp26))))) * fTemp30)) + fTemp171);
			self.fVec17[0] = fTemp172;
			self.fRec68[0] = ((self.fConst1 * (fTemp3 * self.fRec68[1])) + (fSlow36 * fTemp4));
			let mut fTemp173: F32 = F32::tan((self.fConst9 * F32::powf(2.0, (0.0833333358 * self.fRec68[0]))));
			let mut fTemp174: F32 = (1.0 / fTemp173);
			let mut fTemp175: F32 = (fTemp174 + 1.0);
			let mut fTemp176: F32 = (0.0 - (1.0 / (fTemp173 * fTemp175)));
			let mut fTemp177: F32 = (1.0 - fTemp174);
			self.fRec5[0] = ((self.fVec17[1] * fTemp176) + (((fTemp172 / fTemp173) - (self.fRec5[1] * fTemp177)) / fTemp175));
			let mut fTemp178: F32 = (((fTemp174 + -1.0) / fTemp173) + 1.0);
			let mut fTemp179: F32 = insert_faustpower2_f(fTemp173);
			let mut fTemp180: F32 = (1.0 - (1.0 / fTemp179));
			let mut fTemp181: F32 = (((fTemp174 + 1.0) / fTemp173) + 1.0);
			self.fRec4[0] = (self.fRec5[0] - (((self.fRec4[2] * fTemp178) + (2.0 * (self.fRec4[1] * fTemp180))) / fTemp181));
			let mut fTemp182: F32 = (0.0 - (2.0 / fTemp179));
			self.fRec70[0] = (0.0 - (((fTemp177 * self.fRec70[1]) - (fTemp172 + self.fVec17[1])) / fTemp175));
			self.fRec69[0] = (self.fRec70[0] - (((fTemp178 * self.fRec69[2]) + (2.0 * (fTemp180 * self.fRec69[1]))) / fTemp181));
			self.fRec71[0] = ((self.fConst1 * (fTemp3 * self.fRec71[1])) + (fSlow37 * fTemp4));
			let mut fTemp183: F32 = F32::powf(10.0, F32::log10(F32::powf(8.0, (0.0399999991 * self.fRec71[0]))));
			self.fRec72[0] = ((self.fConst1 * (fTemp3 * self.fRec72[1])) + (fSlow38 * fTemp4));
			let mut fTemp184: F32 = F32::powf(2.0, (0.0833333358 * self.fRec72[0]));
			let mut fTemp185: F32 = F32::tan((self.fConst10 * fTemp184));
			let mut fTemp186: F32 = (1.0 / fTemp185);
			self.fRec73[0] = ((self.fConst1 * (fTemp3 * self.fRec73[1])) + (fSlow39 * fTemp4));
			let mut fTemp187: F32 = (20.0 * F32::log10(F32::powf(8.0, (0.0399999991 * self.fRec73[0]))));
			let mut iTemp188: i32 = ((fTemp187 > 0.0) as i32);
			let mut fTemp189: F32 = F32::sin((self.fConst11 * fTemp184));
			let mut fTemp190: F32 = (self.fConst10 * ((fTemp184 * F32::powf(10.0, (0.0500000007 * F32::abs(fTemp187)))) / fTemp189));
			let mut fTemp191: F32 = (self.fConst10 * (fTemp184 / fTemp189));
			let mut fTemp192: F32 = if (iTemp188 as i32 != 0) { fTemp191 } else { fTemp190 };
			let mut fTemp193: F32 = (((fTemp186 - fTemp192) / fTemp185) + 1.0);
			let mut fTemp194: F32 = (1.0 - (1.0 / insert_faustpower2_f(fTemp185)));
			let mut fTemp195: F32 = (2.0 * (self.fRec3[1] * fTemp194));
			let mut fTemp196: F32 = (((fTemp186 + fTemp192) / fTemp185) + 1.0);
			self.fRec3[0] = ((((((self.fRec4[1] * fTemp182) + (self.fRec4[0] / fTemp179)) + (self.fRec4[2] / fTemp179)) + ((self.fRec69[2] + (self.fRec69[0] + (2.0 * self.fRec69[1]))) * fTemp183)) / fTemp181) - (((self.fRec3[2] * fTemp193) + fTemp195) / fTemp196));
			let mut fTemp197: F32 = if (iTemp188 as i32 != 0) { fTemp190 } else { fTemp191 };
			let mut fTemp198: F32 = (((fTemp186 + fTemp197) / fTemp185) + 1.0);
			let mut fTemp199: F32 = (((fTemp186 - fTemp197) / fTemp185) + 1.0);
			let mut fTemp200: F32 = ((fTemp195 + (self.fRec3[0] * fTemp198)) + (self.fRec3[2] * fTemp199));
			let mut fTemp201: F32 = (fTemp200 / fTemp196);
			self.fVec18[0] = fTemp201;
			self.fRec74[0] = ((self.fConst1 * (self.fRec74[1] * fTemp3)) + (fSlow40 * fTemp4));
			self.fRec75[0] = ((self.fConst1 * (fTemp3 * self.fRec75[1])) + (fSlow41 * fTemp4));
			let mut fTemp202: F32 = F32::tan((self.fConst12 * F32::powf(2.0, (0.0833333358 * (self.fRec74[0] - (0.5 * (self.fRec75[0] * fTemp29)))))));
			let mut fTemp203: F32 = (1.0 / fTemp202);
			let mut fTemp204: F32 = (fTemp203 + 1.0);
			let mut fTemp205: F32 = (0.0 - (1.0 / (fTemp202 * fTemp204)));
			let mut fTemp206: F32 = (fTemp202 * fTemp196);
			let mut fTemp207: F32 = (1.0 - fTemp203);
			self.fRec2[0] = ((self.fVec18[1] * fTemp205) + (((fTemp200 / fTemp206) - (self.fRec2[1] * fTemp207)) / fTemp204));
			let mut fTemp208: F32 = (((fTemp203 + -1.0) / fTemp202) + 1.0);
			let mut fTemp209: F32 = insert_faustpower2_f(fTemp202);
			let mut fTemp210: F32 = (1.0 - (1.0 / fTemp209));
			let mut fTemp211: F32 = (((fTemp203 + 1.0) / fTemp202) + 1.0);
			self.fRec1[0] = (self.fRec2[0] - (((self.fRec1[2] * fTemp208) + (2.0 * (self.fRec1[1] * fTemp210))) / fTemp211));
			let mut fTemp212: F32 = (0.0 - (2.0 / fTemp209));
			self.fRec76[0] = ((self.fConst1 * (fTemp3 * self.fRec76[1])) + (fSlow42 * fTemp4));
			let mut fTemp213: F32 = F32::powf(10.0, F32::log10(F32::powf(8.0, (0.0399999991 * self.fRec76[0]))));
			self.fRec78[0] = (0.0 - (((fTemp207 * self.fRec78[1]) - (fTemp201 + self.fVec18[1])) / fTemp204));
			self.fRec77[0] = (self.fRec78[0] - (((fTemp208 * self.fRec77[2]) + (2.0 * (fTemp210 * self.fRec77[1]))) / fTemp211));
			let mut fTemp214: F32 = (((((self.fRec1[1] * fTemp212) + (self.fRec1[0] / fTemp209)) + (self.fRec1[2] / fTemp209)) * fTemp213) + (self.fRec77[2] + (self.fRec77[0] + (2.0 * self.fRec77[1]))));
			self.fRec79[0] = ((self.fConst1 * (fTemp3 * self.fRec79[1])) + (fSlow43 * fTemp4));
			let mut fTemp215: F32 = (0.0399999991 * self.fRec79[0]);
			let mut fTemp216: F32 = F32::max(fTemp215, 0.0);
			let mut fTemp217: F32 = (*input3 as F32);
			let mut fTemp218: F32 = (fTemp217 * fTemp10);
			self.fVec19[(self.IOTA & 131071) as usize] = fTemp218;
			let mut fTemp219: F32 = (fTemp171 + (0.0399999991 * ((self.fRec6[0] * fTemp30) * ((fTemp217 * self.fRec8[0]) + (fTemp10 * ((fTemp18 * ((self.fVec19[((self.IOTA - iTemp13) & 131071) as usize] * fTemp15) + (fTemp16 * self.fVec19[((self.IOTA - iTemp17) & 131071) as usize]))) + (fTemp26 * ((self.fVec19[((self.IOTA - iTemp21) & 131071) as usize] * fTemp23) + (self.fVec19[((self.IOTA - iTemp24) & 131071) as usize] * fTemp25)))))))));
			self.fVec20[0] = fTemp219;
			self.fRec84[0] = ((fTemp176 * self.fVec20[1]) - (((fTemp177 * self.fRec84[1]) - (fTemp219 / fTemp173)) / fTemp175));
			self.fRec83[0] = (self.fRec84[0] - (((fTemp178 * self.fRec83[2]) + (2.0 * (fTemp180 * self.fRec83[1]))) / fTemp181));
			self.fRec86[0] = (0.0 - (((fTemp177 * self.fRec86[1]) - (fTemp219 + self.fVec20[1])) / fTemp175));
			self.fRec85[0] = (self.fRec86[0] - (((fTemp178 * self.fRec85[2]) + (2.0 * (fTemp180 * self.fRec85[1]))) / fTemp181));
			let mut fTemp220: F32 = (2.0 * (fTemp194 * self.fRec82[1]));
			self.fRec82[0] = ((((((fTemp182 * self.fRec83[1]) + (self.fRec83[0] / fTemp179)) + (self.fRec83[2] / fTemp179)) + (fTemp183 * (self.fRec85[2] + (self.fRec85[0] + (2.0 * self.fRec85[1]))))) / fTemp181) - (((fTemp193 * self.fRec82[2]) + fTemp220) / fTemp196));
			let mut fTemp221: F32 = ((fTemp220 + (self.fRec82[0] * fTemp198)) + (fTemp199 * self.fRec82[2]));
			let mut fTemp222: F32 = (fTemp221 / fTemp196);
			self.fVec21[0] = fTemp222;
			self.fRec81[0] = ((fTemp205 * self.fVec21[1]) - (((fTemp207 * self.fRec81[1]) - (fTemp221 / fTemp206)) / fTemp204));
			self.fRec80[0] = (self.fRec81[0] - (((fTemp208 * self.fRec80[2]) + (2.0 * (fTemp210 * self.fRec80[1]))) / fTemp211));
			self.fRec88[0] = (0.0 - (((fTemp207 * self.fRec88[1]) - (fTemp222 + self.fVec21[1])) / fTemp204));
			self.fRec87[0] = (self.fRec88[0] - (((fTemp208 * self.fRec87[2]) + (2.0 * (fTemp210 * self.fRec87[1]))) / fTemp211));
			let mut fTemp223: F32 = ((fTemp213 * (((fTemp212 * self.fRec80[1]) + (self.fRec80[0] / fTemp209)) + (self.fRec80[2] / fTemp209))) + (self.fRec87[2] + (self.fRec87[0] + (2.0 * self.fRec87[1]))));
			let mut fTemp224: F32 = F32::min(fTemp215, 0.0);
			self.fRec89[0] = ((self.fConst1 * (fTemp3 * self.fRec89[1])) + (fSlow44 * fTemp4));
			let mut fTemp225: F32 = F32::powf(10.0, (0.0399999991 * self.fRec89[0]));
			let mut fTemp226: F32 = F32::max(-1.0, F32::min(1.0, (((((fTemp214 * (1.0 - fTemp216)) + (fTemp223 * (0.0 - fTemp224))) * fTemp225) / fTemp211) + 0.5)));
			let mut fTemp227: F32 = (fTemp226 * (1.0 - (0.333333343 * insert_faustpower2_f(fTemp226))));
			self.fVec22[0] = fTemp227;
			self.fRec0[0] = (((0.995000005 * self.fRec0[1]) + fTemp227) - self.fVec22[1]);
			self.fRec90[0] = ((self.fConst1 * (fTemp3 * self.fRec90[1])) + (fSlow45 * fTemp4));
			*output0 = ((0.0199999996 * (self.fRec0[0] * self.fRec90[0])) as F32);
			let mut fTemp228: F32 = F32::max(-1.0, F32::min(1.0, (((fTemp225 * ((fTemp214 * fTemp216) + (fTemp223 * (fTemp224 + 1.0)))) / fTemp211) + 0.5)));
			let mut fTemp229: F32 = (fTemp228 * (1.0 - (0.333333343 * insert_faustpower2_f(fTemp228))));
			self.fVec23[0] = fTemp229;
			self.fRec91[0] = (((0.995000005 * self.fRec91[1]) + fTemp229) - self.fVec23[1]);
			*output1 = ((0.0199999996 * (self.fRec90[0] * self.fRec91[0])) as F32);
			self.fRec92[0] = ((self.fConst1 * (fTemp3 * self.fRec92[1])) + (fSlow46 * fTemp4));
			self.fRec95[0] = ((self.fConst1 * (fTemp3 * self.fRec95[1])) + (fSlow47 * fTemp4));
			let mut fTemp230: F32 = F32::abs(F32::min(1.0, (0.0399999991 * ((*input0 as F32) * self.fRec95[0]))));
			self.fRec96[0] = ((self.fConst1 * (fTemp3 * self.fRec96[1])) + (fSlow48 * fTemp4));
			let mut fTemp231: F32 = (2.0 * insert_faustpower2_f((0.0199999996 * self.fRec96[0])));
			let mut iTemp232: i32 = ((F32::abs(fTemp231) < 1.1920929e-07) as i32);
			let mut fTemp233: F32 = if (iTemp232 as i32 != 0) { 0.0 } else { F32::exp((0.0 - (self.fConst4 / if (iTemp232 as i32 != 0) { 1.0 } else { fTemp231 }))) };
			let mut fTemp234: F32 = if (((self.fRec93[1] > fTemp230) as i32) as i32 != 0) { fTemp233 } else { 0.0 };
			self.fRec94[0] = ((self.fRec94[1] * fTemp234) + (fTemp230 * (1.0 - fTemp234)));
			self.fRec93[0] = self.fRec94[0];
			let mut fTemp235: F32 = (1.0 - self.fRec93[0]);
			*output2 = ((0.0199999996 * ((self.fRec0[0] * self.fRec92[0]) * fTemp235)) as F32);
			let mut fTemp236: F32 = F32::abs(F32::min(1.0, (0.0399999991 * ((*input1 as F32) * self.fRec95[0]))));
			let mut fTemp237: F32 = if (((self.fRec97[1] > fTemp236) as i32) as i32 != 0) { fTemp233 } else { 0.0 };
			self.fRec98[0] = ((self.fRec98[1] * fTemp237) + (fTemp236 * (1.0 - fTemp237)));
			self.fRec97[0] = self.fRec98[0];
			let mut fTemp238: F32 = (1.0 - self.fRec97[0]);
			*output3 = ((0.0199999996 * ((self.fRec91[0] * self.fRec92[0]) * fTemp238)) as F32);
			self.fRec99[0] = ((self.fConst1 * (fTemp3 * self.fRec99[1])) + (fSlow49 * fTemp4));
			*output4 = ((0.0199999996 * ((self.fRec0[0] * self.fRec99[0]) * fTemp235)) as F32);
			*output5 = ((0.0199999996 * ((self.fRec91[0] * self.fRec99[0]) * fTemp238)) as F32);
			self.fRec100[0] = ((self.fConst1 * (fTemp3 * self.fRec100[1])) + (fSlow50 * fTemp4));
			*output6 = ((0.0199999996 * ((self.fRec0[0] * self.fRec100[0]) * fTemp235)) as F32);
			*output7 = ((0.0199999996 * ((self.fRec91[0] * self.fRec100[0]) * fTemp238)) as F32);
			self.iVec0[1] = self.iVec0[0];
			self.fVec1[1] = self.fVec1[0];
			self.iRec7[1] = self.iRec7[0];
			self.iVec2[1] = self.iVec2[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec8[1] = self.fRec8[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec10[1] = self.fRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.iRec15[1] = self.iRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec20[1] = self.fRec20[0];
			self.fVec4[1] = self.fVec4[0];
			self.iRec23[1] = self.iRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec43[1] = self.fRec43[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec48[1] = self.fRec48[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec59[1] = self.fRec59[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec64[1] = self.fRec64[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec67[1] = self.fRec67[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec69[2] = self.fRec69[1];
			self.fRec69[1] = self.fRec69[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec73[1] = self.fRec73[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fVec18[1] = self.fVec18[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec1[2] = self.fRec1[1];
			self.fRec1[1] = self.fRec1[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec77[2] = self.fRec77[1];
			self.fRec77[1] = self.fRec77[0];
			self.fRec79[1] = self.fRec79[0];
			self.fVec20[1] = self.fVec20[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec83[2] = self.fRec83[1];
			self.fRec83[1] = self.fRec83[0];
			self.fRec86[1] = self.fRec86[0];
			self.fRec85[2] = self.fRec85[1];
			self.fRec85[1] = self.fRec85[0];
			self.fRec82[2] = self.fRec82[1];
			self.fRec82[1] = self.fRec82[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec80[2] = self.fRec80[1];
			self.fRec80[1] = self.fRec80[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec87[2] = self.fRec87[1];
			self.fRec87[1] = self.fRec87[0];
			self.fRec89[1] = self.fRec89[0];
			self.fVec22[1] = self.fVec22[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec90[1] = self.fRec90[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec94[1] = self.fRec94[0];
			self.fRec93[1] = self.fRec93[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec99[1] = self.fRec99[0];
			self.fRec100[1] = self.fRec100[0];
		}
	}

}

