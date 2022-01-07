

pub struct insertSIG0 {
	iVec4: [i32;2],
	iRec21: [i32;2],
}

impl insertSIG0 {
	
	fn get_num_inputsinsertSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsinsertSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initinsertSIG0(&mut self, sample_rate: i32) {
		let mut l19: i32 = 0;
		loop {
			self.iVec4[l19 as usize] = 0;
			l19 = (l19 + 1);
			if (l19 < 2) { continue; } else { break; }
		}
		let mut l20: i32 = 0;
		loop {
			self.iRec21[l20 as usize] = 0;
			l20 = (l20 + 1);
			if (l20 < 2) { continue; } else { break; }
		}
	}
	
	fn fillinsertSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec4[0] = 1;
			self.iRec21[0] = ((self.iVec4[1] + self.iRec21[1]) % 65536);
			table[i1 as usize] = F32::sin((9.58738019e-05 * (self.iRec21[0] as F32)));
			self.iVec4[1] = self.iVec4[0];
			self.iRec21[1] = self.iRec21[0];
		}
	}

}


pub fn newinsertSIG0() -> insertSIG0 { 
	insertSIG0 {
		iVec4: [0;2],
		iRec21: [0;2],
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
	fRec19: [F32;2],
	fRec20: [F32;2],
	fConst3: F32,
	fEntry10: F32,
	fRec23: [F32;2],
	fRec22: [F32;2],
	fConst4: F32,
	fConst5: F32,
	fConst6: F32,
	fRec25: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fConst7: F32,
	fRec24: [F32;2],
	fRec26: [F32;2],
	fConst8: F32,
	fRec28: [F32;2],
	iRec29: [i32;2],
	fEntry11: F32,
	fRec30: [F32;2],
	fEntry12: F32,
	fRec31: [F32;2],
	fRec32: [F32;2],
	fRec33: [F32;2],
	fEntry13: F32,
	fRec35: [F32;2],
	fRec34: [F32;2],
	fRec37: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec36: [F32;2],
	fRec38: [F32;2],
	fRec40: [F32;2],
	fEntry14: F32,
	fRec41: [F32;2],
	fEntry15: F32,
	fRec42: [F32;2],
	fRec43: [F32;2],
	fRec44: [F32;2],
	fEntry16: F32,
	fRec46: [F32;2],
	fRec45: [F32;2],
	fRec48: [F32;2],
	fVec9: [F32;2],
	fVec10: [F32;4096],
	fRec47: [F32;2],
	fRec49: [F32;2],
	fRec51: [F32;2],
	fVec11: [F32;2],
	fConst9: F32,
	fEntry17: F32,
	fRec52: [F32;2],
	fRec5: [F32;2],
	fRec4: [F32;3],
	fRec54: [F32;2],
	fRec53: [F32;3],
	fEntry18: F32,
	fRec55: [F32;2],
	fConst10: F32,
	fEntry19: F32,
	fRec56: [F32;2],
	fEntry20: F32,
	fRec57: [F32;2],
	fConst11: F32,
	fRec3: [F32;3],
	fVec12: [F32;2],
	fConst12: F32,
	fEntry21: F32,
	fRec58: [F32;2],
	fEntry22: F32,
	fRec59: [F32;2],
	fRec2: [F32;2],
	fRec1: [F32;3],
	fEntry23: F32,
	fRec60: [F32;2],
	fRec62: [F32;2],
	fRec61: [F32;3],
	fEntry24: F32,
	fRec63: [F32;2],
	fVec13: [F32;131072],
	fVec14: [F32;2],
	fRec68: [F32;2],
	fRec67: [F32;3],
	fRec70: [F32;2],
	fRec69: [F32;3],
	fRec66: [F32;3],
	fVec15: [F32;2],
	fRec65: [F32;2],
	fRec64: [F32;3],
	fRec72: [F32;2],
	fRec71: [F32;3],
	fEntry25: F32,
	fRec73: [F32;2],
	fVec16: [F32;2],
	fRec0: [F32;2],
	fEntry26: F32,
	fRec74: [F32;2],
	fVec17: [F32;2],
	fRec75: [F32;2],
	fEntry27: F32,
	fRec76: [F32;2],
	fEntry28: F32,
	fRec79: [F32;2],
	fButton2: F32,
	fRec80: [F32;2],
	fRec78: [F32;2],
	fRec77: [F32;2],
	fRec82: [F32;2],
	fRec81: [F32;2],
	fEntry29: F32,
	fRec83: [F32;2],
	fEntry30: F32,
	fRec84: [F32;2],
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
			fRec19: [0.0;2],
			fRec20: [0.0;2],
			fConst3: 0.0,
			fEntry10: 0.0,
			fRec23: [0.0;2],
			fRec22: [0.0;2],
			fConst4: 0.0,
			fConst5: 0.0,
			fConst6: 0.0,
			fRec25: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fConst7: 0.0,
			fRec24: [0.0;2],
			fRec26: [0.0;2],
			fConst8: 0.0,
			fRec28: [0.0;2],
			iRec29: [0;2],
			fEntry11: 0.0,
			fRec30: [0.0;2],
			fEntry12: 0.0,
			fRec31: [0.0;2],
			fRec32: [0.0;2],
			fRec33: [0.0;2],
			fEntry13: 0.0,
			fRec35: [0.0;2],
			fRec34: [0.0;2],
			fRec37: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec36: [0.0;2],
			fRec38: [0.0;2],
			fRec40: [0.0;2],
			fEntry14: 0.0,
			fRec41: [0.0;2],
			fEntry15: 0.0,
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fRec44: [0.0;2],
			fEntry16: 0.0,
			fRec46: [0.0;2],
			fRec45: [0.0;2],
			fRec48: [0.0;2],
			fVec9: [0.0;2],
			fVec10: [0.0;4096],
			fRec47: [0.0;2],
			fRec49: [0.0;2],
			fRec51: [0.0;2],
			fVec11: [0.0;2],
			fConst9: 0.0,
			fEntry17: 0.0,
			fRec52: [0.0;2],
			fRec5: [0.0;2],
			fRec4: [0.0;3],
			fRec54: [0.0;2],
			fRec53: [0.0;3],
			fEntry18: 0.0,
			fRec55: [0.0;2],
			fConst10: 0.0,
			fEntry19: 0.0,
			fRec56: [0.0;2],
			fEntry20: 0.0,
			fRec57: [0.0;2],
			fConst11: 0.0,
			fRec3: [0.0;3],
			fVec12: [0.0;2],
			fConst12: 0.0,
			fEntry21: 0.0,
			fRec58: [0.0;2],
			fEntry22: 0.0,
			fRec59: [0.0;2],
			fRec2: [0.0;2],
			fRec1: [0.0;3],
			fEntry23: 0.0,
			fRec60: [0.0;2],
			fRec62: [0.0;2],
			fRec61: [0.0;3],
			fEntry24: 0.0,
			fRec63: [0.0;2],
			fVec13: [0.0;131072],
			fVec14: [0.0;2],
			fRec68: [0.0;2],
			fRec67: [0.0;3],
			fRec70: [0.0;2],
			fRec69: [0.0;3],
			fRec66: [0.0;3],
			fVec15: [0.0;2],
			fRec65: [0.0;2],
			fRec64: [0.0;3],
			fRec72: [0.0;2],
			fRec71: [0.0;3],
			fEntry25: 0.0,
			fRec73: [0.0;2],
			fVec16: [0.0;2],
			fRec0: [0.0;2],
			fEntry26: 0.0,
			fRec74: [0.0;2],
			fVec17: [0.0;2],
			fRec75: [0.0;2],
			fEntry27: 0.0,
			fRec76: [0.0;2],
			fEntry28: 0.0,
			fRec79: [0.0;2],
			fButton2: 0.0,
			fRec80: [0.0;2],
			fRec78: [0.0;2],
			fRec77: [0.0;2],
			fRec82: [0.0;2],
			fRec81: [0.0;2],
			fEntry29: 0.0,
			fRec83: [0.0;2],
			fEntry30: 0.0,
			fRec84: [0.0;2],
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
		self.fEntry10 = 0.0;
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
		self.fEntry27 = 50.0;
		self.fEntry28 = 0.0;
		self.fButton2 = 0.0;
		self.fEntry29 = 0.0;
		self.fEntry30 = 0.0;
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
			self.fRec20[l18 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec23[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec22[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec25[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fVec5[l24 as usize] = 0.0;
		}
		for l25 in 0..4096 {
			self.fVec6[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec24[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec26[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec28[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.iRec29[l29 as usize] = 0;
		}
		for l30 in 0..2 {
			self.fRec30[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec31[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec32[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec33[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec35[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec34[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec37[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fVec7[l37 as usize] = 0.0;
		}
		for l38 in 0..4096 {
			self.fVec8[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec36[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec38[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec40[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec41[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec42[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec43[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec44[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec46[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec45[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec48[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fVec9[l49 as usize] = 0.0;
		}
		for l50 in 0..4096 {
			self.fVec10[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec47[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec49[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec51[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec11[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec52[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec5[l56 as usize] = 0.0;
		}
		for l57 in 0..3 {
			self.fRec4[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec54[l58 as usize] = 0.0;
		}
		for l59 in 0..3 {
			self.fRec53[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec55[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec56[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec57[l62 as usize] = 0.0;
		}
		for l63 in 0..3 {
			self.fRec3[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fVec12[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec58[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec59[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec2[l67 as usize] = 0.0;
		}
		for l68 in 0..3 {
			self.fRec1[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec60[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec62[l70 as usize] = 0.0;
		}
		for l71 in 0..3 {
			self.fRec61[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec63[l72 as usize] = 0.0;
		}
		for l73 in 0..131072 {
			self.fVec13[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fVec14[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec68[l75 as usize] = 0.0;
		}
		for l76 in 0..3 {
			self.fRec67[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec70[l77 as usize] = 0.0;
		}
		for l78 in 0..3 {
			self.fRec69[l78 as usize] = 0.0;
		}
		for l79 in 0..3 {
			self.fRec66[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fVec15[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec65[l81 as usize] = 0.0;
		}
		for l82 in 0..3 {
			self.fRec64[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec72[l83 as usize] = 0.0;
		}
		for l84 in 0..3 {
			self.fRec71[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec73[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fVec16[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec0[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec74[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fVec17[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec75[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec76[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec79[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec80[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec78[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec77[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec82[l96 as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec81[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec83[l98 as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec84[l99 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32)));
		self.fConst1 = (1.0 - (44.0999985 / self.fConst0));
		self.fConst2 = (2.0 * self.fConst0);
		self.fConst3 = (440.0 / self.fConst0);
		self.fConst4 = (1760.0 / self.fConst0);
		self.fConst5 = (0.25 * self.fConst0);
		self.fConst6 = (1.0 / self.fConst0);
		self.fConst7 = (0.5 * self.fConst0);
		self.fConst8 = (0.125 * self.fConst0);
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
		ui_interface.add_num_entry("sustain", ParamIndex(23), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth1Detune", ParamIndex(24), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth1Level", ParamIndex(25), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth1Type", ParamIndex(26), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("synth2Detune", ParamIndex(27), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth2Level", ParamIndex(28), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth2Type", ParamIndex(29), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("synth3Detune", ParamIndex(30), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth3Level", ParamIndex(31), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth3Type", ParamIndex(32), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("toDuck", ParamIndex(33), 0.0, 0.0, 50.0, 10.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			7 => Some(self.fButton0),
			16 => Some(self.fButton1),
			5 => Some(self.fButton2),
			18 => Some(self.fEntry0),
			21 => Some(self.fEntry1),
			27 => Some(self.fEntry10),
			25 => Some(self.fEntry11),
			26 => Some(self.fEntry12),
			24 => Some(self.fEntry13),
			31 => Some(self.fEntry14),
			32 => Some(self.fEntry15),
			30 => Some(self.fEntry16),
			11 => Some(self.fEntry17),
			12 => Some(self.fEntry18),
			14 => Some(self.fEntry19),
			22 => Some(self.fEntry2),
			15 => Some(self.fEntry20),
			8 => Some(self.fEntry21),
			1 => Some(self.fEntry22),
			9 => Some(self.fEntry23),
			17 => Some(self.fEntry24),
			3 => Some(self.fEntry25),
			33 => Some(self.fEntry26),
			13 => Some(self.fEntry27),
			4 => Some(self.fEntry28),
			6 => Some(self.fEntry29),
			20 => Some(self.fEntry3),
			19 => Some(self.fEntry30),
			10 => Some(self.fEntry4),
			0 => Some(self.fEntry5),
			23 => Some(self.fEntry6),
			2 => Some(self.fEntry7),
			28 => Some(self.fEntry8),
			29 => Some(self.fEntry9),
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
			27 => { self.fEntry10 = value }
			25 => { self.fEntry11 = value }
			26 => { self.fEntry12 = value }
			24 => { self.fEntry13 = value }
			31 => { self.fEntry14 = value }
			32 => { self.fEntry15 = value }
			30 => { self.fEntry16 = value }
			11 => { self.fEntry17 = value }
			12 => { self.fEntry18 = value }
			14 => { self.fEntry19 = value }
			22 => { self.fEntry2 = value }
			15 => { self.fEntry20 = value }
			8 => { self.fEntry21 = value }
			1 => { self.fEntry22 = value }
			9 => { self.fEntry23 = value }
			17 => { self.fEntry24 = value }
			3 => { self.fEntry25 = value }
			33 => { self.fEntry26 = value }
			13 => { self.fEntry27 = value }
			4 => { self.fEntry28 = value }
			6 => { self.fEntry29 = value }
			20 => { self.fEntry3 = value }
			19 => { self.fEntry30 = value }
			10 => { self.fEntry4 = value }
			0 => { self.fEntry5 = value }
			23 => { self.fEntry6 = value }
			2 => { self.fEntry7 = value }
			28 => { self.fEntry8 = value }
			29 => { self.fEntry9 = value }
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
		let mut fSlow17: F32 = (((fSlow15 >= 2.0) as i32) as F32);
		let mut fSlow18: F32 = (((fSlow15 >= 1.0) as i32) as F32);
		let mut fSlow19: F32 = (self.fEntry10 as F32);
		let mut fSlow20: F32 = (((fSlow15 >= 4.0) as i32) as F32);
		let mut fSlow21: F32 = (self.fEntry11 as F32);
		let mut fSlow22: F32 = (self.fEntry12 as F32);
		let mut fSlow23: F32 = (((fSlow22 >= 3.0) as i32) as F32);
		let mut fSlow24: F32 = (((fSlow22 >= 2.0) as i32) as F32);
		let mut fSlow25: F32 = (((fSlow22 >= 1.0) as i32) as F32);
		let mut fSlow26: F32 = (self.fEntry13 as F32);
		let mut fSlow27: F32 = (((fSlow22 >= 4.0) as i32) as F32);
		let mut fSlow28: F32 = (self.fEntry14 as F32);
		let mut fSlow29: F32 = (self.fEntry15 as F32);
		let mut fSlow30: F32 = (((fSlow29 >= 3.0) as i32) as F32);
		let mut fSlow31: F32 = (((fSlow29 >= 2.0) as i32) as F32);
		let mut fSlow32: F32 = (((fSlow29 >= 1.0) as i32) as F32);
		let mut fSlow33: F32 = (self.fEntry16 as F32);
		let mut fSlow34: F32 = (((fSlow29 >= 4.0) as i32) as F32);
		let mut fSlow35: F32 = (self.fEntry17 as F32);
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
		let mut fSlow47: F32 = (self.fButton2 as F32);
		let mut fSlow48: F32 = (self.fEntry29 as F32);
		let mut fSlow49: F32 = (self.fEntry30 as F32);
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
			let mut fTemp35: F32 = (self.fRec20[1] + 1.0);
			let mut fTemp36: F32 = (self.fRec20[1] + -1.0);
			self.fRec20[0] = if (((fTemp35 < fSlow18) as i32) as i32 != 0) { fTemp35 } else { if (((fTemp36 > fSlow18) as i32) as i32 != 0) { fTemp36 } else { fSlow18 } };
			self.fRec23[0] = ((self.fConst1 * (fTemp3 * self.fRec23[1])) + (fSlow19 * fTemp4));
			let mut fTemp37: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((0.00999999978 * self.fRec23[0]) + -69.0))));
			let mut fTemp38: F32 = (self.fRec22[1] + (self.fConst3 * fTemp37));
			self.fRec22[0] = (fTemp38 - F32::floor(fTemp38));
			let mut fTemp39: F32 = (self.iVec0[1] as F32);
			let mut fTemp40: F32 = (440.0 * fTemp37);
			let mut fTemp41: F32 = F32::max(fTemp40, 23.4489498);
			let mut fTemp42: F32 = F32::max(20.0, F32::abs(fTemp41));
			let mut fTemp43: F32 = (self.fRec25[1] + (self.fConst6 * fTemp42));
			self.fRec25[0] = (fTemp43 - F32::floor(fTemp43));
			let mut fTemp44: F32 = insert_faustpower2_f(((2.0 * self.fRec25[0]) + -1.0));
			self.fVec5[0] = fTemp44;
			let mut fTemp45: F32 = ((fTemp39 * (fTemp44 - self.fVec5[1])) / fTemp42);
			self.fVec6[(self.IOTA & 4095) as usize] = fTemp45;
			let mut fTemp46: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst7 / fTemp41)));
			let mut iTemp47: i32 = (fTemp46 as i32);
			let mut fTemp48: F32 = F32::floor(fTemp46);
			let mut fTemp49: F32 = ((fTemp45 - (self.fVec6[((self.IOTA - iTemp47) & 4095) as usize] * (fTemp48 + (1.0 - fTemp46)))) - ((fTemp46 - fTemp48) * self.fVec6[((self.IOTA - (iTemp47 + 1)) & 4095) as usize]));
			self.fRec24[0] = ((0.999000013 * self.fRec24[1]) + (self.fConst5 * fTemp49));
			let mut fTemp50: F32 = F32::max(1.1920929e-07, F32::abs(fTemp40));
			let mut fTemp51: F32 = (self.fRec26[1] + (self.fConst6 * fTemp50));
			let mut fTemp52: F32 = (fTemp51 + -1.0);
			let mut iTemp53: i32 = ((fTemp52 < 0.0) as i32);
			self.fRec26[0] = if (iTemp53 as i32 != 0) { fTemp51 } else { fTemp52 };
			let mut fRec27: F32 = if (iTemp53 as i32 != 0) { fTemp51 } else { (fTemp51 + (fTemp52 * (1.0 - (self.fConst0 / fTemp50)))) };
			let mut fTemp54: F32 = (self.fRec28[1] + 1.0);
			let mut fTemp55: F32 = (self.fRec28[1] + -1.0);
			self.fRec28[0] = if (((fTemp54 < fSlow20) as i32) as i32 != 0) { fTemp54 } else { if (((fTemp55 > fSlow20) as i32) as i32 != 0) { fTemp55 } else { fSlow20 } };
			self.iRec29[0] = ((1103515245 * self.iRec29[1]) + 12345);
			let mut fTemp56: F32 = (self.iRec29[0] as F32);
			self.fRec30[0] = ((self.fConst1 * (fTemp3 * self.fRec30[1])) + (fSlow21 * fTemp4));
			let mut fTemp57: F32 = (self.fRec31[1] + 1.0);
			let mut fTemp58: F32 = (self.fRec31[1] + -1.0);
			self.fRec31[0] = if (((fTemp57 < fSlow23) as i32) as i32 != 0) { fTemp57 } else { if (((fTemp58 > fSlow23) as i32) as i32 != 0) { fTemp58 } else { fSlow23 } };
			let mut fTemp59: F32 = (self.fRec32[1] + 1.0);
			let mut fTemp60: F32 = (self.fRec32[1] + -1.0);
			self.fRec32[0] = if (((fTemp59 < fSlow24) as i32) as i32 != 0) { fTemp59 } else { if (((fTemp60 > fSlow24) as i32) as i32 != 0) { fTemp60 } else { fSlow24 } };
			let mut fTemp61: F32 = (self.fRec33[1] + 1.0);
			let mut fTemp62: F32 = (self.fRec33[1] + -1.0);
			self.fRec33[0] = if (((fTemp61 < fSlow25) as i32) as i32 != 0) { fTemp61 } else { if (((fTemp62 > fSlow25) as i32) as i32 != 0) { fTemp62 } else { fSlow25 } };
			self.fRec35[0] = ((self.fConst1 * (fTemp3 * self.fRec35[1])) + (fSlow26 * fTemp4));
			let mut fTemp63: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((0.00999999978 * self.fRec35[0]) + -69.0))));
			let mut fTemp64: F32 = (self.fRec34[1] + (self.fConst3 * fTemp63));
			self.fRec34[0] = (fTemp64 - F32::floor(fTemp64));
			let mut fTemp65: F32 = (440.0 * fTemp63);
			let mut fTemp66: F32 = F32::max(fTemp65, 23.4489498);
			let mut fTemp67: F32 = F32::max(20.0, F32::abs(fTemp66));
			let mut fTemp68: F32 = (self.fRec37[1] + (self.fConst6 * fTemp67));
			self.fRec37[0] = (fTemp68 - F32::floor(fTemp68));
			let mut fTemp69: F32 = insert_faustpower2_f(((2.0 * self.fRec37[0]) + -1.0));
			self.fVec7[0] = fTemp69;
			let mut fTemp70: F32 = ((fTemp39 * (fTemp69 - self.fVec7[1])) / fTemp67);
			self.fVec8[(self.IOTA & 4095) as usize] = fTemp70;
			let mut fTemp71: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst7 / fTemp66)));
			let mut iTemp72: i32 = (fTemp71 as i32);
			let mut fTemp73: F32 = F32::floor(fTemp71);
			let mut fTemp74: F32 = (self.fConst5 * (((self.fVec8[((self.IOTA - iTemp72) & 4095) as usize] * (fTemp73 + (1.0 - fTemp71))) - fTemp70) + ((fTemp71 - fTemp73) * self.fVec8[((self.IOTA - (iTemp72 + 1)) & 4095) as usize])));
			self.fRec36[0] = ((0.999000013 * self.fRec36[1]) - fTemp74);
			let mut fTemp75: F32 = F32::max(1.1920929e-07, F32::abs(fTemp65));
			let mut fTemp76: F32 = (self.fRec38[1] + (self.fConst6 * fTemp75));
			let mut fTemp77: F32 = (fTemp76 + -1.0);
			let mut iTemp78: i32 = ((fTemp77 < 0.0) as i32);
			self.fRec38[0] = if (iTemp78 as i32 != 0) { fTemp76 } else { fTemp77 };
			let mut fRec39: F32 = if (iTemp78 as i32 != 0) { fTemp76 } else { (fTemp76 + (fTemp77 * (1.0 - (self.fConst0 / fTemp75)))) };
			let mut fTemp79: F32 = (self.fRec40[1] + 1.0);
			let mut fTemp80: F32 = (self.fRec40[1] + -1.0);
			self.fRec40[0] = if (((fTemp79 < fSlow27) as i32) as i32 != 0) { fTemp79 } else { if (((fTemp80 > fSlow27) as i32) as i32 != 0) { fTemp80 } else { fSlow27 } };
			self.fRec41[0] = ((self.fConst1 * (fTemp3 * self.fRec41[1])) + (fSlow28 * fTemp4));
			let mut fTemp81: F32 = (self.fRec42[1] + 1.0);
			let mut fTemp82: F32 = (self.fRec42[1] + -1.0);
			self.fRec42[0] = if (((fTemp81 < fSlow30) as i32) as i32 != 0) { fTemp81 } else { if (((fTemp82 > fSlow30) as i32) as i32 != 0) { fTemp82 } else { fSlow30 } };
			let mut fTemp83: F32 = (self.fRec43[1] + 1.0);
			let mut fTemp84: F32 = (self.fRec43[1] + -1.0);
			self.fRec43[0] = if (((fTemp83 < fSlow31) as i32) as i32 != 0) { fTemp83 } else { if (((fTemp84 > fSlow31) as i32) as i32 != 0) { fTemp84 } else { fSlow31 } };
			let mut fTemp85: F32 = (self.fRec44[1] + 1.0);
			let mut fTemp86: F32 = (self.fRec44[1] + -1.0);
			self.fRec44[0] = if (((fTemp85 < fSlow32) as i32) as i32 != 0) { fTemp85 } else { if (((fTemp86 > fSlow32) as i32) as i32 != 0) { fTemp86 } else { fSlow32 } };
			self.fRec46[0] = ((self.fConst1 * (fTemp3 * self.fRec46[1])) + (fSlow33 * fTemp4));
			let mut fTemp87: F32 = F32::powf(2.0, (0.0833333358 * (fSlow6 + ((0.00999999978 * self.fRec46[0]) + -69.0))));
			let mut fTemp88: F32 = (self.fRec45[1] + (self.fConst3 * fTemp87));
			self.fRec45[0] = (fTemp88 - F32::floor(fTemp88));
			let mut fTemp89: F32 = (440.0 * fTemp87);
			let mut fTemp90: F32 = F32::max(fTemp89, 23.4489498);
			let mut fTemp91: F32 = F32::max(20.0, F32::abs(fTemp90));
			let mut fTemp92: F32 = (self.fRec48[1] + (self.fConst6 * fTemp91));
			self.fRec48[0] = (fTemp92 - F32::floor(fTemp92));
			let mut fTemp93: F32 = insert_faustpower2_f(((2.0 * self.fRec48[0]) + -1.0));
			self.fVec9[0] = fTemp93;
			let mut fTemp94: F32 = ((fTemp39 * (fTemp93 - self.fVec9[1])) / fTemp91);
			self.fVec10[(self.IOTA & 4095) as usize] = fTemp94;
			let mut fTemp95: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst7 / fTemp90)));
			let mut iTemp96: i32 = (fTemp95 as i32);
			let mut fTemp97: F32 = F32::floor(fTemp95);
			let mut fTemp98: F32 = ((fTemp94 - (self.fVec10[((self.IOTA - iTemp96) & 4095) as usize] * (fTemp97 + (1.0 - fTemp95)))) - ((fTemp95 - fTemp97) * self.fVec10[((self.IOTA - (iTemp96 + 1)) & 4095) as usize]));
			self.fRec47[0] = ((0.999000013 * self.fRec47[1]) + (self.fConst5 * fTemp98));
			let mut fTemp99: F32 = F32::max(1.1920929e-07, F32::abs(fTemp89));
			let mut fTemp100: F32 = (self.fRec49[1] + (self.fConst6 * fTemp99));
			let mut fTemp101: F32 = (fTemp100 + -1.0);
			let mut iTemp102: i32 = ((fTemp101 < 0.0) as i32);
			self.fRec49[0] = if (iTemp102 as i32 != 0) { fTemp100 } else { fTemp101 };
			let mut fRec50: F32 = if (iTemp102 as i32 != 0) { fTemp100 } else { (fTemp100 + (fTemp101 * (1.0 - (self.fConst0 / fTemp99)))) };
			let mut fTemp103: F32 = (self.fRec51[1] + 1.0);
			let mut fTemp104: F32 = (self.fRec51[1] + -1.0);
			self.fRec51[0] = if (((fTemp103 < fSlow34) as i32) as i32 != 0) { fTemp103 } else { if (((fTemp104 > fSlow34) as i32) as i32 != 0) { fTemp104 } else { fSlow34 } };
			let mut fTemp105: F32 = (0.00666666683 * (fTemp29 * (((self.fRec17[0] * (((1.0 - self.fRec18[0]) * (((1.0 - self.fRec19[0]) * (((1.0 - self.fRec20[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec22[0]) as i32) as usize] }) + (self.fConst4 * ((self.fRec20[0] * self.fRec24[0]) * fTemp37)))) + (0.5 * (self.fRec19[0] * ((2.0 * fRec27) + -1.0))))) + (self.fRec18[0] * ((self.fConst8 * ((1.0 - self.fRec28[0]) * fTemp49)) + (1.16415322e-10 * (self.fRec28[0] * fTemp56)))))) + (self.fRec30[0] * (((1.0 - self.fRec31[0]) * (((1.0 - self.fRec32[0]) * (((1.0 - self.fRec33[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec34[0]) as i32) as usize] }) + (self.fConst4 * ((self.fRec33[0] * self.fRec36[0]) * fTemp63)))) + (0.5 * (self.fRec32[0] * ((2.0 * fRec39) + -1.0))))) + (self.fRec31[0] * ((0.5 * ((1.0 - self.fRec40[0]) * (0.0 - fTemp74))) + (1.16415322e-10 * (self.fRec40[0] * fTemp56))))))) + (self.fRec41[0] * (((1.0 - self.fRec42[0]) * (((1.0 - self.fRec43[0]) * (((1.0 - self.fRec44[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec45[0]) as i32) as usize] }) + (self.fConst4 * ((self.fRec44[0] * self.fRec47[0]) * fTemp87)))) + (0.5 * (self.fRec43[0] * ((2.0 * fRec50) + -1.0))))) + (self.fRec42[0] * ((self.fConst8 * ((1.0 - self.fRec51[0]) * fTemp98)) + (1.16415322e-10 * (self.fRec51[0] * fTemp56)))))))));
			let mut fTemp106: F32 = ((0.0399999991 * ((self.fRec6[0] * ((fTemp5 * self.fRec8[0]) + (fTemp10 * ((((self.fVec3[((self.IOTA - iTemp13) & 131071) as usize] * fTemp15) + (fTemp16 * self.fVec3[((self.IOTA - iTemp17) & 131071) as usize])) * fTemp18) + (((self.fVec3[((self.IOTA - iTemp21) & 131071) as usize] * fTemp23) + (self.fVec3[((self.IOTA - iTemp24) & 131071) as usize] * fTemp25)) * fTemp26))))) * fTemp30)) + fTemp105);
			self.fVec11[0] = fTemp106;
			self.fRec52[0] = ((self.fConst1 * (fTemp3 * self.fRec52[1])) + (fSlow35 * fTemp4));
			let mut fTemp107: F32 = F32::tan((self.fConst9 * F32::powf(2.0, (0.0833333358 * self.fRec52[0]))));
			let mut fTemp108: F32 = (1.0 / fTemp107);
			let mut fTemp109: F32 = (fTemp108 + 1.0);
			let mut fTemp110: F32 = (0.0 - (1.0 / (fTemp107 * fTemp109)));
			let mut fTemp111: F32 = (1.0 - fTemp108);
			self.fRec5[0] = ((self.fVec11[1] * fTemp110) - (((self.fRec5[1] * fTemp111) - (fTemp106 / fTemp107)) / fTemp109));
			let mut fTemp112: F32 = (((fTemp108 + -1.0) / fTemp107) + 1.0);
			let mut fTemp113: F32 = insert_faustpower2_f(fTemp107);
			let mut fTemp114: F32 = (1.0 - (1.0 / fTemp113));
			let mut fTemp115: F32 = (((fTemp108 + 1.0) / fTemp107) + 1.0);
			self.fRec4[0] = (self.fRec5[0] - (((self.fRec4[2] * fTemp112) + (2.0 * (self.fRec4[1] * fTemp114))) / fTemp115));
			let mut fTemp116: F32 = (0.0 - (2.0 / fTemp113));
			self.fRec54[0] = (0.0 - (((fTemp111 * self.fRec54[1]) - (fTemp106 + self.fVec11[1])) / fTemp109));
			self.fRec53[0] = (self.fRec54[0] - (((fTemp112 * self.fRec53[2]) + (2.0 * (fTemp114 * self.fRec53[1]))) / fTemp115));
			self.fRec55[0] = ((self.fConst1 * (fTemp3 * self.fRec55[1])) + (fSlow36 * fTemp4));
			let mut fTemp117: F32 = F32::powf(10.0, F32::log10(F32::powf(8.0, (0.0399999991 * self.fRec55[0]))));
			self.fRec56[0] = ((self.fConst1 * (fTemp3 * self.fRec56[1])) + (fSlow37 * fTemp4));
			let mut fTemp118: F32 = F32::powf(2.0, (0.0833333358 * self.fRec56[0]));
			let mut fTemp119: F32 = F32::tan((self.fConst10 * fTemp118));
			let mut fTemp120: F32 = (1.0 / fTemp119);
			self.fRec57[0] = ((self.fConst1 * (fTemp3 * self.fRec57[1])) + (fSlow38 * fTemp4));
			let mut fTemp121: F32 = (20.0 * F32::log10(F32::powf(8.0, (0.0399999991 * self.fRec57[0]))));
			let mut iTemp122: i32 = ((fTemp121 > 0.0) as i32);
			let mut fTemp123: F32 = F32::sin((self.fConst11 * fTemp118));
			let mut fTemp124: F32 = (self.fConst10 * ((fTemp118 * F32::powf(10.0, (0.0500000007 * F32::abs(fTemp121)))) / fTemp123));
			let mut fTemp125: F32 = (self.fConst10 * (fTemp118 / fTemp123));
			let mut fTemp126: F32 = if (iTemp122 as i32 != 0) { fTemp125 } else { fTemp124 };
			let mut fTemp127: F32 = (((fTemp120 - fTemp126) / fTemp119) + 1.0);
			let mut fTemp128: F32 = (1.0 - (1.0 / insert_faustpower2_f(fTemp119)));
			let mut fTemp129: F32 = (2.0 * (self.fRec3[1] * fTemp128));
			let mut fTemp130: F32 = (((fTemp120 + fTemp126) / fTemp119) + 1.0);
			self.fRec3[0] = ((((((self.fRec4[1] * fTemp116) + (self.fRec4[0] / fTemp113)) + (self.fRec4[2] / fTemp113)) + ((self.fRec53[2] + (self.fRec53[0] + (2.0 * self.fRec53[1]))) * fTemp117)) / fTemp115) - (((self.fRec3[2] * fTemp127) + fTemp129) / fTemp130));
			let mut fTemp131: F32 = if (iTemp122 as i32 != 0) { fTemp124 } else { fTemp125 };
			let mut fTemp132: F32 = (((fTemp120 + fTemp131) / fTemp119) + 1.0);
			let mut fTemp133: F32 = (((fTemp120 - fTemp131) / fTemp119) + 1.0);
			let mut fTemp134: F32 = ((fTemp129 + (self.fRec3[0] * fTemp132)) + (self.fRec3[2] * fTemp133));
			let mut fTemp135: F32 = (fTemp134 / fTemp130);
			self.fVec12[0] = fTemp135;
			self.fRec58[0] = ((self.fConst1 * (self.fRec58[1] * fTemp3)) + (fSlow39 * fTemp4));
			self.fRec59[0] = ((self.fConst1 * (fTemp3 * self.fRec59[1])) + (fSlow40 * fTemp4));
			let mut fTemp136: F32 = F32::tan((self.fConst12 * F32::powf(2.0, (0.0833333358 * (self.fRec58[0] - (0.5 * (self.fRec59[0] * fTemp29)))))));
			let mut fTemp137: F32 = (1.0 / fTemp136);
			let mut fTemp138: F32 = (fTemp137 + 1.0);
			let mut fTemp139: F32 = (0.0 - (1.0 / (fTemp136 * fTemp138)));
			let mut fTemp140: F32 = (1.0 - fTemp137);
			let mut fTemp141: F32 = (fTemp136 * fTemp130);
			self.fRec2[0] = ((self.fVec12[1] * fTemp139) - (((self.fRec2[1] * fTemp140) - (fTemp134 / fTemp141)) / fTemp138));
			let mut fTemp142: F32 = (((fTemp137 + -1.0) / fTemp136) + 1.0);
			let mut fTemp143: F32 = insert_faustpower2_f(fTemp136);
			let mut fTemp144: F32 = (1.0 - (1.0 / fTemp143));
			let mut fTemp145: F32 = (((fTemp137 + 1.0) / fTemp136) + 1.0);
			self.fRec1[0] = (self.fRec2[0] - (((self.fRec1[2] * fTemp142) + (2.0 * (self.fRec1[1] * fTemp144))) / fTemp145));
			let mut fTemp146: F32 = (0.0 - (2.0 / fTemp143));
			self.fRec60[0] = ((self.fConst1 * (fTemp3 * self.fRec60[1])) + (fSlow41 * fTemp4));
			let mut fTemp147: F32 = F32::powf(10.0, F32::log10(F32::powf(8.0, (0.0399999991 * self.fRec60[0]))));
			self.fRec62[0] = (0.0 - (((fTemp140 * self.fRec62[1]) - (fTemp135 + self.fVec12[1])) / fTemp138));
			self.fRec61[0] = (self.fRec62[0] - (((fTemp142 * self.fRec61[2]) + (2.0 * (fTemp144 * self.fRec61[1]))) / fTemp145));
			let mut fTemp148: F32 = (((((self.fRec1[1] * fTemp146) + (self.fRec1[0] / fTemp143)) + (self.fRec1[2] / fTemp143)) * fTemp147) + (self.fRec61[2] + (self.fRec61[0] + (2.0 * self.fRec61[1]))));
			self.fRec63[0] = ((self.fConst1 * (fTemp3 * self.fRec63[1])) + (fSlow42 * fTemp4));
			let mut fTemp149: F32 = (0.0399999991 * self.fRec63[0]);
			let mut fTemp150: F32 = F32::max(fTemp149, 0.0);
			let mut fTemp151: F32 = (*input3 as F32);
			let mut fTemp152: F32 = (fTemp151 * fTemp10);
			self.fVec13[(self.IOTA & 131071) as usize] = fTemp152;
			let mut fTemp153: F32 = (fTemp105 + (0.0399999991 * ((self.fRec6[0] * fTemp30) * ((fTemp151 * self.fRec8[0]) + (fTemp10 * ((fTemp18 * ((self.fVec13[((self.IOTA - iTemp13) & 131071) as usize] * fTemp15) + (fTemp16 * self.fVec13[((self.IOTA - iTemp17) & 131071) as usize]))) + (fTemp26 * ((self.fVec13[((self.IOTA - iTemp21) & 131071) as usize] * fTemp23) + (self.fVec13[((self.IOTA - iTemp24) & 131071) as usize] * fTemp25)))))))));
			self.fVec14[0] = fTemp153;
			self.fRec68[0] = ((fTemp110 * self.fVec14[1]) - (((fTemp111 * self.fRec68[1]) - (fTemp153 / fTemp107)) / fTemp109));
			self.fRec67[0] = (self.fRec68[0] - (((fTemp112 * self.fRec67[2]) + (2.0 * (fTemp114 * self.fRec67[1]))) / fTemp115));
			self.fRec70[0] = (((fTemp153 + self.fVec14[1]) - (fTemp111 * self.fRec70[1])) / fTemp109);
			self.fRec69[0] = (self.fRec70[0] - (((fTemp112 * self.fRec69[2]) + (2.0 * (fTemp114 * self.fRec69[1]))) / fTemp115));
			let mut fTemp154: F32 = (2.0 * (fTemp128 * self.fRec66[1]));
			self.fRec66[0] = ((((((fTemp116 * self.fRec67[1]) + (self.fRec67[0] / fTemp113)) + (self.fRec67[2] / fTemp113)) + (fTemp117 * (self.fRec69[2] + (self.fRec69[0] + (2.0 * self.fRec69[1]))))) / fTemp115) - (((fTemp127 * self.fRec66[2]) + fTemp154) / fTemp130));
			let mut fTemp155: F32 = ((fTemp154 + (self.fRec66[0] * fTemp132)) + (fTemp133 * self.fRec66[2]));
			let mut fTemp156: F32 = (fTemp155 / fTemp130);
			self.fVec15[0] = fTemp156;
			self.fRec65[0] = ((fTemp139 * self.fVec15[1]) + (((fTemp155 / fTemp141) - (fTemp140 * self.fRec65[1])) / fTemp138));
			self.fRec64[0] = (self.fRec65[0] - (((fTemp142 * self.fRec64[2]) + (2.0 * (fTemp144 * self.fRec64[1]))) / fTemp145));
			self.fRec72[0] = (0.0 - (((fTemp140 * self.fRec72[1]) - (fTemp156 + self.fVec15[1])) / fTemp138));
			self.fRec71[0] = (self.fRec72[0] - (((fTemp142 * self.fRec71[2]) + (2.0 * (fTemp144 * self.fRec71[1]))) / fTemp145));
			let mut fTemp157: F32 = ((fTemp147 * (((fTemp146 * self.fRec64[1]) + (self.fRec64[0] / fTemp143)) + (self.fRec64[2] / fTemp143))) + (self.fRec71[2] + (self.fRec71[0] + (2.0 * self.fRec71[1]))));
			let mut fTemp158: F32 = F32::min(fTemp149, 0.0);
			self.fRec73[0] = ((self.fConst1 * (fTemp3 * self.fRec73[1])) + (fSlow43 * fTemp4));
			let mut fTemp159: F32 = F32::powf(10.0, (0.0399999991 * self.fRec73[0]));
			let mut fTemp160: F32 = F32::max(-1.0, F32::min(1.0, (((((fTemp148 * (1.0 - fTemp150)) + (fTemp157 * (0.0 - fTemp158))) * fTemp159) / fTemp145) + 0.5)));
			let mut fTemp161: F32 = (fTemp160 * (1.0 - (0.333333343 * insert_faustpower2_f(fTemp160))));
			self.fVec16[0] = fTemp161;
			self.fRec0[0] = (((0.995000005 * self.fRec0[1]) + fTemp161) - self.fVec16[1]);
			self.fRec74[0] = ((self.fConst1 * (fTemp3 * self.fRec74[1])) + (fSlow44 * fTemp4));
			*output0 = ((0.0199999996 * (self.fRec0[0] * self.fRec74[0])) as F32);
			let mut fTemp162: F32 = F32::max(-1.0, F32::min(1.0, (((fTemp159 * ((fTemp148 * fTemp150) + (fTemp157 * (fTemp158 + 1.0)))) / fTemp145) + 0.5)));
			let mut fTemp163: F32 = (fTemp162 * (1.0 - (0.333333343 * insert_faustpower2_f(fTemp162))));
			self.fVec17[0] = fTemp163;
			self.fRec75[0] = (((0.995000005 * self.fRec75[1]) + fTemp163) - self.fVec17[1]);
			*output1 = ((0.0199999996 * (self.fRec74[0] * self.fRec75[0])) as F32);
			self.fRec76[0] = ((self.fConst1 * (fTemp3 * self.fRec76[1])) + (fSlow45 * fTemp4));
			self.fRec79[0] = ((self.fConst1 * (fTemp3 * self.fRec79[1])) + (fSlow46 * fTemp4));
			let mut fTemp164: F32 = F32::abs(F32::min(1.0, (0.0399999991 * ((*input0 as F32) * self.fRec79[0]))));
			self.fRec80[0] = ((self.fConst1 * (fTemp3 * self.fRec80[1])) + (fSlow47 * fTemp4));
			let mut fTemp165: F32 = (2.0 * insert_faustpower2_f((0.0199999996 * self.fRec80[0])));
			let mut iTemp166: i32 = ((F32::abs(fTemp165) < 1.1920929e-07) as i32);
			let mut fTemp167: F32 = if (iTemp166 as i32 != 0) { 0.0 } else { F32::exp((0.0 - (self.fConst6 / if (iTemp166 as i32 != 0) { 1.0 } else { fTemp165 }))) };
			let mut fTemp168: F32 = if (((self.fRec77[1] > fTemp164) as i32) as i32 != 0) { fTemp167 } else { 0.0 };
			self.fRec78[0] = ((self.fRec78[1] * fTemp168) + (fTemp164 * (1.0 - fTemp168)));
			self.fRec77[0] = self.fRec78[0];
			let mut fTemp169: F32 = (1.0 - self.fRec77[0]);
			*output2 = ((0.0199999996 * ((self.fRec0[0] * self.fRec76[0]) * fTemp169)) as F32);
			let mut fTemp170: F32 = F32::abs(F32::min(1.0, (0.0399999991 * ((*input1 as F32) * self.fRec79[0]))));
			let mut fTemp171: F32 = if (((self.fRec81[1] > fTemp170) as i32) as i32 != 0) { fTemp167 } else { 0.0 };
			self.fRec82[0] = ((self.fRec82[1] * fTemp171) + (fTemp170 * (1.0 - fTemp171)));
			self.fRec81[0] = self.fRec82[0];
			let mut fTemp172: F32 = (1.0 - self.fRec81[0]);
			*output3 = ((0.0199999996 * ((self.fRec75[0] * self.fRec76[0]) * fTemp172)) as F32);
			self.fRec83[0] = ((self.fConst1 * (fTemp3 * self.fRec83[1])) + (fSlow48 * fTemp4));
			*output4 = ((0.0199999996 * ((self.fRec0[0] * self.fRec83[0]) * fTemp169)) as F32);
			*output5 = ((0.0199999996 * ((self.fRec75[0] * self.fRec83[0]) * fTemp172)) as F32);
			self.fRec84[0] = ((self.fConst1 * (fTemp3 * self.fRec84[1])) + (fSlow49 * fTemp4));
			*output6 = ((0.0199999996 * ((self.fRec0[0] * self.fRec84[0]) * fTemp169)) as F32);
			*output7 = ((0.0199999996 * ((self.fRec75[0] * self.fRec84[0]) * fTemp172)) as F32);
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
			self.fRec20[1] = self.fRec20[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec25[1] = self.fRec25[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec28[1] = self.fRec28[0];
			self.iRec29[1] = self.iRec29[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec37[1] = self.fRec37[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec48[1] = self.fRec48[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec51[1] = self.fRec51[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec53[2] = self.fRec53[1];
			self.fRec53[1] = self.fRec53[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec1[2] = self.fRec1[1];
			self.fRec1[1] = self.fRec1[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec61[2] = self.fRec61[1];
			self.fRec61[1] = self.fRec61[0];
			self.fRec63[1] = self.fRec63[0];
			self.fVec14[1] = self.fVec14[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec67[2] = self.fRec67[1];
			self.fRec67[1] = self.fRec67[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec69[2] = self.fRec69[1];
			self.fRec69[1] = self.fRec69[0];
			self.fRec66[2] = self.fRec66[1];
			self.fRec66[1] = self.fRec66[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec64[2] = self.fRec64[1];
			self.fRec64[1] = self.fRec64[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec71[2] = self.fRec71[1];
			self.fRec71[1] = self.fRec71[0];
			self.fRec73[1] = self.fRec73[0];
			self.fVec16[1] = self.fVec16[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec74[1] = self.fRec74[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec79[1] = self.fRec79[0];
			self.fRec80[1] = self.fRec80[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec84[1] = self.fRec84[0];
		}
	}

}

