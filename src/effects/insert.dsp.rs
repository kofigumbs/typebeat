

pub struct insertSIG0 {
	iVec3: [i32;2],
	iRec28: [i32;2],
}

impl insertSIG0 {
	
	fn get_num_inputsinsertSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsinsertSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initinsertSIG0(&mut self, sample_rate: i32) {
		let mut l28: i32 = 0;
		loop {
			self.iVec3[l28 as usize] = 0;
			l28 = (l28 + 1);
			if (l28 < 2) { continue; } else { break; }
		}
		let mut l29: i32 = 0;
		loop {
			self.iRec28[l29 as usize] = 0;
			l29 = (l29 + 1);
			if (l29 < 2) { continue; } else { break; }
		}
	}
	
	fn fillinsertSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec3[0] = 1;
			self.iRec28[0] = ((self.iVec3[1] + self.iRec28[1]) % 65536);
			table[i1 as usize] = F32::sin((9.58738019e-05 * (self.iRec28[0] as F32)));
			self.iVec3[1] = self.iVec3[0];
			self.iRec28[1] = self.iRec28[0];
		}
	}

}


pub fn newinsertSIG0() -> insertSIG0 { 
	insertSIG0 {
		iVec3: [0;2],
		iRec28: [0;2],
	}
}
static mut ftbl0insertSIG0: [F32;65536] = [0.0;65536];
fn insert_faustpower2_f(value: F32) -> F32 {
	return (value * value);
}
pub struct insert {
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	iVec0: [i32;2],
	fButton0: F32,
	fVec1: [F32;2],
	fRec1: [F32;2],
	fEntry0: F32,
	fRec0: [F32;2],
	fEntry1: F32,
	fRec2: [F32;2],
	fEntry2: F32,
	fRec4: [F32;2],
	fEntry3: F32,
	fRec5: [F32;2],
	fRec6: [F32;2],
	fConst2: F32,
	fEntry4: F32,
	fRec7: [F32;2],
	fEntry5: F32,
	fRec8: [F32;2],
	fEntry6: F32,
	fRec9: [F32;2],
	iRec10: [i32;2],
	fEntry7: F32,
	fRec11: [F32;2],
	fRec3: [F32;2],
	fConst3: F32,
	fEntry8: F32,
	fRec14: [F32;2],
	fEntry9: F32,
	fRec15: [F32;2],
	fRec13: [F32;2],
	fEntry10: F32,
	fRec18: [F32;2],
	fRec17: [F32;2],
	fEntry11: F32,
	fRec20: [F32;2],
	fEntry12: F32,
	fButton1: F32,
	fEntry13: F32,
	fRec22: [F32;2],
	fRec21: [F32;2],
	IOTA: i32,
	fVec2: [F32;131072],
	fRec23: [F32;2],
	fEntry14: F32,
	fEntry15: F32,
	fRec24: [F32;2],
	fEntry16: F32,
	fRec25: [F32;2],
	fRec26: [F32;2],
	fRec27: [F32;2],
	fConst4: F32,
	fEntry17: F32,
	fRec30: [F32;2],
	fRec29: [F32;2],
	fConst5: F32,
	fConst6: F32,
	fConst7: F32,
	fRec32: [F32;2],
	fVec4: [F32;2],
	fVec5: [F32;4096],
	fConst8: F32,
	fRec31: [F32;2],
	fRec33: [F32;2],
	fConst9: F32,
	fRec35: [F32;2],
	iRec36: [i32;2],
	fEntry18: F32,
	fRec37: [F32;2],
	fEntry19: F32,
	fRec38: [F32;2],
	fRec39: [F32;2],
	fRec40: [F32;2],
	fEntry20: F32,
	fRec42: [F32;2],
	fRec41: [F32;2],
	fRec44: [F32;2],
	fVec6: [F32;2],
	fVec7: [F32;4096],
	fRec43: [F32;2],
	fRec45: [F32;2],
	fRec47: [F32;2],
	fEntry21: F32,
	fRec48: [F32;2],
	fEntry22: F32,
	fRec49: [F32;2],
	fRec50: [F32;2],
	fRec51: [F32;2],
	fEntry23: F32,
	fRec53: [F32;2],
	fRec52: [F32;2],
	fRec55: [F32;2],
	fVec8: [F32;2],
	fVec9: [F32;4096],
	fRec54: [F32;2],
	fRec56: [F32;2],
	fRec58: [F32;2],
	fEntry24: F32,
	fRec59: [F32;2],
	fRec19: [F32;3],
	fConst10: F32,
	fRec16: [F32;3],
	fEntry25: F32,
	fRec60: [F32;2],
	fRec12: [F32;3],
	fVec10: [F32;131072],
	fRec63: [F32;3],
	fRec62: [F32;3],
	fRec61: [F32;3],
	fEntry26: F32,
	fRec64: [F32;2],
	fEntry27: F32,
	fRec65: [F32;2],
	fEntry28: F32,
	fRec66: [F32;2],
}

impl FaustDsp for insert {
	type T = F32;
		
	fn new() -> insert { 
		insert {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			iVec0: [0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			fRec1: [0.0;2],
			fEntry0: 0.0,
			fRec0: [0.0;2],
			fEntry1: 0.0,
			fRec2: [0.0;2],
			fEntry2: 0.0,
			fRec4: [0.0;2],
			fEntry3: 0.0,
			fRec5: [0.0;2],
			fRec6: [0.0;2],
			fConst2: 0.0,
			fEntry4: 0.0,
			fRec7: [0.0;2],
			fEntry5: 0.0,
			fRec8: [0.0;2],
			fEntry6: 0.0,
			fRec9: [0.0;2],
			iRec10: [0;2],
			fEntry7: 0.0,
			fRec11: [0.0;2],
			fRec3: [0.0;2],
			fConst3: 0.0,
			fEntry8: 0.0,
			fRec14: [0.0;2],
			fEntry9: 0.0,
			fRec15: [0.0;2],
			fRec13: [0.0;2],
			fEntry10: 0.0,
			fRec18: [0.0;2],
			fRec17: [0.0;2],
			fEntry11: 0.0,
			fRec20: [0.0;2],
			fEntry12: 0.0,
			fButton1: 0.0,
			fEntry13: 0.0,
			fRec22: [0.0;2],
			fRec21: [0.0;2],
			IOTA: 0,
			fVec2: [0.0;131072],
			fRec23: [0.0;2],
			fEntry14: 0.0,
			fEntry15: 0.0,
			fRec24: [0.0;2],
			fEntry16: 0.0,
			fRec25: [0.0;2],
			fRec26: [0.0;2],
			fRec27: [0.0;2],
			fConst4: 0.0,
			fEntry17: 0.0,
			fRec30: [0.0;2],
			fRec29: [0.0;2],
			fConst5: 0.0,
			fConst6: 0.0,
			fConst7: 0.0,
			fRec32: [0.0;2],
			fVec4: [0.0;2],
			fVec5: [0.0;4096],
			fConst8: 0.0,
			fRec31: [0.0;2],
			fRec33: [0.0;2],
			fConst9: 0.0,
			fRec35: [0.0;2],
			iRec36: [0;2],
			fEntry18: 0.0,
			fRec37: [0.0;2],
			fEntry19: 0.0,
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fRec40: [0.0;2],
			fEntry20: 0.0,
			fRec42: [0.0;2],
			fRec41: [0.0;2],
			fRec44: [0.0;2],
			fVec6: [0.0;2],
			fVec7: [0.0;4096],
			fRec43: [0.0;2],
			fRec45: [0.0;2],
			fRec47: [0.0;2],
			fEntry21: 0.0,
			fRec48: [0.0;2],
			fEntry22: 0.0,
			fRec49: [0.0;2],
			fRec50: [0.0;2],
			fRec51: [0.0;2],
			fEntry23: 0.0,
			fRec53: [0.0;2],
			fRec52: [0.0;2],
			fRec55: [0.0;2],
			fVec8: [0.0;2],
			fVec9: [0.0;4096],
			fRec54: [0.0;2],
			fRec56: [0.0;2],
			fRec58: [0.0;2],
			fEntry24: 0.0,
			fRec59: [0.0;2],
			fRec19: [0.0;3],
			fConst10: 0.0,
			fRec16: [0.0;3],
			fEntry25: 0.0,
			fRec60: [0.0;2],
			fRec12: [0.0;3],
			fVec10: [0.0;131072],
			fRec63: [0.0;3],
			fRec62: [0.0;3],
			fRec61: [0.0;3],
			fEntry26: 0.0,
			fRec64: [0.0;2],
			fEntry27: 0.0,
			fRec65: [0.0;2],
			fEntry28: 0.0,
			fRec66: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("envelopes.lib/adsr:author", "Yann Orlarey and Andrey Bundin");
		m.declare("envelopes.lib/author", "GRAME");
		m.declare("envelopes.lib/copyright", "GRAME");
		m.declare("envelopes.lib/license", "LGPL with exception");
		m.declare("envelopes.lib/name", "Faust Envelope Library");
		m.declare("envelopes.lib/version", "0.1");
		m.declare("filename", "insert.dsp");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/pole:author", "Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/version", "0.3");
		m.declare("interpolators.lib/name", "Faust Interpolator Library");
		m.declare("interpolators.lib/version", "0.3");
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
		m.declare("webaudio.lib/author", "GRAME");
		m.declare("webaudio.lib/copyright", "GRAME");
		m.declare("webaudio.lib/license", "LGPL with exception");
		m.declare("webaudio.lib/name", "WebAudio Filters Library");
		m.declare("webaudio.lib/version", "0.1");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 2;
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
		self.fEntry0 = 50.0;
		self.fEntry1 = 0.0;
		self.fEntry2 = 50.0;
		self.fEntry3 = 0.0;
		self.fEntry4 = 0.0;
		self.fEntry5 = 50.0;
		self.fEntry6 = 0.0;
		self.fEntry7 = 0.0;
		self.fEntry8 = 0.0;
		self.fEntry9 = 0.0;
		self.fEntry10 = 0.0;
		self.fEntry11 = 25.0;
		self.fEntry12 = 0.0;
		self.fButton1 = 0.0;
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
		self.fEntry28 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fVec1[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec1[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec0[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec2[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec4[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec5[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec6[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec7[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec8[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec9[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.iRec10[l11 as usize] = 0;
		}
		for l12 in 0..2 {
			self.fRec11[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec3[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec14[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec15[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec13[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec18[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec17[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec20[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec22[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec21[l21 as usize] = 0.0;
		}
		self.IOTA = 0;
		for l22 in 0..131072 {
			self.fVec2[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec23[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec24[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec25[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec26[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec27[l27 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec30[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec29[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec32[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec4[l33 as usize] = 0.0;
		}
		for l34 in 0..4096 {
			self.fVec5[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec31[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec33[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec35[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.iRec36[l38 as usize] = 0;
		}
		for l39 in 0..2 {
			self.fRec37[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec38[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec39[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec40[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec42[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec41[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec44[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec6[l46 as usize] = 0.0;
		}
		for l47 in 0..4096 {
			self.fVec7[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec43[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec45[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec47[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec48[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec49[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec50[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec51[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec53[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec52[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec55[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fVec8[l58 as usize] = 0.0;
		}
		for l59 in 0..4096 {
			self.fVec9[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec54[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec56[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec58[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec59[l63 as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fRec19[l64 as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec16[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec60[l66 as usize] = 0.0;
		}
		for l67 in 0..3 {
			self.fRec12[l67 as usize] = 0.0;
		}
		for l68 in 0..131072 {
			self.fVec10[l68 as usize] = 0.0;
		}
		for l69 in 0..3 {
			self.fRec63[l69 as usize] = 0.0;
		}
		for l70 in 0..3 {
			self.fRec62[l70 as usize] = 0.0;
		}
		for l71 in 0..3 {
			self.fRec61[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec64[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec65[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec66[l74 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32)));
		self.fConst1 = (1.0 - (44.0999985 / self.fConst0));
		self.fConst2 = (0.0199999996 * self.fConst0);
		self.fConst3 = (880.0 / self.fConst0);
		self.fConst4 = (440.0 / self.fConst0);
		self.fConst5 = (1760.0 / self.fConst0);
		self.fConst6 = (0.25 * self.fConst0);
		self.fConst7 = (1.0 / self.fConst0);
		self.fConst8 = (0.5 * self.fConst0);
		self.fConst9 = (0.125 * self.fConst0);
		self.fConst10 = (2.0 / self.fConst0);
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
		ui_interface.add_num_entry("echo", ParamIndex(4), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_button("gate", ParamIndex(5));
		ui_interface.add_num_entry("highFreq", ParamIndex(6), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("highRes", ParamIndex(7), 0.0, -50.0, 0.0, 10.0);
		ui_interface.add_num_entry("holdSample", ParamIndex(8), 0.0, 0.0, 1.0, 0.0);
		ui_interface.add_num_entry("lowFreq", ParamIndex(9), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("lowRes", ParamIndex(10), 0.0, -50.0, 0.0, 10.0);
		ui_interface.add_num_entry("main", ParamIndex(11), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("midFreq", ParamIndex(12), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("midRes", ParamIndex(13), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_button("note", ParamIndex(14));
		ui_interface.add_num_entry("pan", ParamIndex(15), 0.0, -25.0, 25.0, 10.0);
		ui_interface.add_num_entry("release", ParamIndex(16), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("reverb", ParamIndex(17), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("sampleDetune", ParamIndex(18), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("sampleLevel", ParamIndex(19), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("sampleType", ParamIndex(20), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("sustain", ParamIndex(21), 50.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth1Detune", ParamIndex(22), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth1Level", ParamIndex(23), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth1Type", ParamIndex(24), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("synth2Detune", ParamIndex(25), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth2Level", ParamIndex(26), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth2Type", ParamIndex(27), 0.0, 0.0, 4.0, 1.0);
		ui_interface.add_num_entry("synth3Detune", ParamIndex(28), 0.0, -120.0, 120.0, 10.0);
		ui_interface.add_num_entry("synth3Level", ParamIndex(29), 0.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("synth3Type", ParamIndex(30), 0.0, 0.0, 4.0, 1.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			5 => Some(self.fButton0),
			14 => Some(self.fButton1),
			11 => Some(self.fEntry0),
			15 => Some(self.fEntry1),
			9 => Some(self.fEntry10),
			19 => Some(self.fEntry11),
			20 => Some(self.fEntry12),
			18 => Some(self.fEntry13),
			8 => Some(self.fEntry14),
			23 => Some(self.fEntry15),
			24 => Some(self.fEntry16),
			22 => Some(self.fEntry17),
			26 => Some(self.fEntry18),
			27 => Some(self.fEntry19),
			6 => Some(self.fEntry2),
			25 => Some(self.fEntry20),
			29 => Some(self.fEntry21),
			30 => Some(self.fEntry22),
			28 => Some(self.fEntry23),
			10 => Some(self.fEntry24),
			7 => Some(self.fEntry25),
			17 => Some(self.fEntry26),
			4 => Some(self.fEntry27),
			3 => Some(self.fEntry28),
			1 => Some(self.fEntry3),
			0 => Some(self.fEntry4),
			21 => Some(self.fEntry5),
			2 => Some(self.fEntry6),
			16 => Some(self.fEntry7),
			12 => Some(self.fEntry8),
			13 => Some(self.fEntry9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			5 => { self.fButton0 = value }
			14 => { self.fButton1 = value }
			11 => { self.fEntry0 = value }
			15 => { self.fEntry1 = value }
			9 => { self.fEntry10 = value }
			19 => { self.fEntry11 = value }
			20 => { self.fEntry12 = value }
			18 => { self.fEntry13 = value }
			8 => { self.fEntry14 = value }
			23 => { self.fEntry15 = value }
			24 => { self.fEntry16 = value }
			22 => { self.fEntry17 = value }
			26 => { self.fEntry18 = value }
			27 => { self.fEntry19 = value }
			6 => { self.fEntry2 = value }
			25 => { self.fEntry20 = value }
			29 => { self.fEntry21 = value }
			30 => { self.fEntry22 = value }
			28 => { self.fEntry23 = value }
			10 => { self.fEntry24 = value }
			7 => { self.fEntry25 = value }
			17 => { self.fEntry26 = value }
			4 => { self.fEntry27 = value }
			3 => { self.fEntry28 = value }
			1 => { self.fEntry3 = value }
			0 => { self.fEntry4 = value }
			21 => { self.fEntry5 = value }
			2 => { self.fEntry6 = value }
			16 => { self.fEntry7 = value }
			12 => { self.fEntry8 = value }
			13 => { self.fEntry9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			(inputs0, inputs1)
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
		let mut fSlow2: F32 = (self.fEntry1 as F32);
		let mut fSlow3: F32 = (self.fEntry2 as F32);
		let mut fSlow4: F32 = (self.fEntry3 as F32);
		let mut fSlow5: F32 = (self.fEntry4 as F32);
		let mut fSlow6: F32 = (self.fEntry5 as F32);
		let mut fSlow7: F32 = (self.fEntry6 as F32);
		let mut iSlow8: i32 = ((fSlow0 == 0.0) as i32);
		let mut fSlow9: F32 = (self.fEntry7 as F32);
		let mut fSlow10: F32 = (self.fEntry8 as F32);
		let mut fSlow11: F32 = (self.fEntry9 as F32);
		let mut fSlow12: F32 = (self.fEntry10 as F32);
		let mut fSlow13: F32 = (self.fEntry11 as F32);
		let mut fSlow14: F32 = (self.fEntry12 as F32);
		let mut fSlow15: F32 = (((((fSlow14 == ((1 | (fSlow14 as i32)) as F32)) as i32) == 2) as i32) as F32);
		let mut fSlow16: F32 = (self.fButton1 as F32);
		let mut fSlow17: F32 = (self.fEntry13 as F32);
		let mut fSlow18: F32 = (0.0833333358 * fSlow15);
		let mut iSlow19: i32 = ((self.fEntry14 as F32) as i32);
		let mut fSlow20: F32 = (self.fEntry15 as F32);
		let mut fSlow21: F32 = (self.fEntry16 as F32);
		let mut fSlow22: F32 = (((fSlow21 >= 3.0) as i32) as F32);
		let mut fSlow23: F32 = (((fSlow21 >= 2.0) as i32) as F32);
		let mut fSlow24: F32 = (((fSlow21 >= 1.0) as i32) as F32);
		let mut fSlow25: F32 = (self.fEntry17 as F32);
		let mut fSlow26: F32 = (((fSlow21 >= 4.0) as i32) as F32);
		let mut fSlow27: F32 = (self.fEntry18 as F32);
		let mut fSlow28: F32 = (self.fEntry19 as F32);
		let mut fSlow29: F32 = (((fSlow28 >= 3.0) as i32) as F32);
		let mut fSlow30: F32 = (((fSlow28 >= 2.0) as i32) as F32);
		let mut fSlow31: F32 = (((fSlow28 >= 1.0) as i32) as F32);
		let mut fSlow32: F32 = (self.fEntry20 as F32);
		let mut fSlow33: F32 = (((fSlow28 >= 4.0) as i32) as F32);
		let mut fSlow34: F32 = (self.fEntry21 as F32);
		let mut fSlow35: F32 = (self.fEntry22 as F32);
		let mut fSlow36: F32 = (((fSlow35 >= 3.0) as i32) as F32);
		let mut fSlow37: F32 = (((fSlow35 >= 2.0) as i32) as F32);
		let mut fSlow38: F32 = (((fSlow35 >= 1.0) as i32) as F32);
		let mut fSlow39: F32 = (self.fEntry23 as F32);
		let mut fSlow40: F32 = (((fSlow35 >= 4.0) as i32) as F32);
		let mut fSlow41: F32 = (self.fEntry24 as F32);
		let mut fSlow42: F32 = (self.fEntry25 as F32);
		let mut fSlow43: F32 = (self.fEntry26 as F32);
		let mut fSlow44: F32 = (self.fEntry27 as F32);
		let mut fSlow45: F32 = (self.fEntry28 as F32);
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1).zip(outputs2).zip(outputs3).zip(outputs4).zip(outputs5).zip(outputs6).zip(outputs7);
		for (((((((((input0, input1), output0), output1), output2), output3), output4), output5), output6), output7) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fVec1[0] = fSlow0;
			self.fRec1[0] = F32::max(self.fRec1[1], fSlow0);
			let mut fTemp0: F32 = ((((self.fRec1[0] == self.fRec1[1]) as i32) | ((self.fRec1[0] == 0.0) as i32)) as F32);
			let mut fTemp1: F32 = (1.0 - (self.fConst1 * fTemp0));
			self.fRec0[0] = ((self.fConst1 * (fTemp0 * self.fRec0[1])) + (fSlow1 * fTemp1));
			self.fRec2[0] = ((self.fConst1 * (self.fRec2[1] * fTemp0)) + (fSlow2 * fTemp1));
			let mut iTemp2: i32 = ((self.fRec2[0] > 25.0) as i32);
			let mut fTemp3: F32 = (self.fRec3[1] + 1.0);
			self.fRec4[0] = ((self.fConst1 * (fTemp0 * self.fRec4[1])) + (fSlow3 * fTemp1));
			self.fRec5[0] = ((self.fConst1 * (fTemp0 * self.fRec5[1])) + (fSlow4 * fTemp1));
			self.fRec6[0] = (fSlow0 + (self.fRec6[1] * (((self.fVec1[1] >= fSlow0) as i32) as F32)));
			self.fRec7[0] = ((self.fConst1 * (fTemp0 * self.fRec7[1])) + (fSlow5 * fTemp1));
			let mut fTemp4: F32 = F32::max(1.0, (self.fConst2 * self.fRec7[0]));
			self.fRec8[0] = ((self.fConst1 * (fTemp0 * self.fRec8[1])) + (fSlow6 * fTemp1));
			let mut fTemp5: F32 = (0.0199999996 * self.fRec8[0]);
			self.fRec9[0] = ((self.fConst1 * (fTemp0 * self.fRec9[1])) + (fSlow7 * fTemp1));
			self.iRec10[0] = (iSlow8 * (self.iRec10[1] + 1));
			self.fRec11[0] = ((self.fConst1 * (fTemp0 * self.fRec11[1])) + (fSlow9 * fTemp1));
			let mut fTemp6: F32 = F32::max(0.0, (F32::min((self.fRec6[0] / fTemp4), F32::max(((((1.0 - fTemp5) * (fTemp4 - self.fRec6[0])) / F32::max(1.0, (self.fConst2 * self.fRec9[0]))) + 1.0), fTemp5)) * (1.0 - ((self.iRec10[0] as F32) / F32::max(1.0, (self.fConst2 * self.fRec11[0]))))));
			let mut fTemp7: F32 = (self.fRec5[0] * fTemp6);
			let mut fTemp8: F32 = ((((self.fRec4[0] - fTemp7) == 50.0) as i32) as F32);
			let mut fTemp9: F32 = (self.fRec3[1] + -1.0);
			self.fRec3[0] = if (((fTemp3 < fTemp8) as i32) as i32 != 0) { fTemp3 } else { if (((fTemp9 > fTemp8) as i32) as i32 != 0) { fTemp9 } else { fTemp8 } };
			let mut fTemp10: F32 = (1.0 - self.fRec3[0]);
			let mut fTemp11: F32 = (3.14159274 * F32::max(0.00999999978, F32::min((self.fConst3 * F32::powf(2.0, (0.0833333358 * (self.fRec4[0] + (-9.0 - fTemp7))))), 0.99000001)));
			let mut fTemp12: F32 = F32::cos(fTemp11);
			let mut fTemp13: F32 = (fTemp10 * (1.0 - fTemp12));
			let mut fTemp14: F32 = (self.fRec13[1] + 1.0);
			self.fRec14[0] = ((self.fConst1 * (fTemp0 * self.fRec14[1])) + (fSlow10 * fTemp1));
			self.fRec15[0] = ((self.fConst1 * (fTemp0 * self.fRec15[1])) + (fSlow11 * fTemp1));
			let mut fTemp15: F32 = ((((self.fRec14[0] == 0.0) as i32) & ((self.fRec15[0] == 0.0) as i32)) as F32);
			let mut fTemp16: F32 = (self.fRec13[1] + -1.0);
			self.fRec13[0] = if (((fTemp14 < fTemp15) as i32) as i32 != 0) { fTemp14 } else { if (((fTemp16 > fTemp15) as i32) as i32 != 0) { fTemp16 } else { fTemp15 } };
			let mut fTemp17: F32 = (1.0 - self.fRec13[0]);
			let mut fTemp18: F32 = (self.fRec17[1] + 1.0);
			self.fRec18[0] = ((self.fConst1 * (fTemp0 * self.fRec18[1])) + (fSlow12 * fTemp1));
			let mut fTemp19: F32 = (((self.fRec18[0] == 0.0) as i32) as F32);
			let mut fTemp20: F32 = (self.fRec17[1] + -1.0);
			self.fRec17[0] = if (((fTemp18 < fTemp19) as i32) as i32 != 0) { fTemp18 } else { if (((fTemp20 > fTemp19) as i32) as i32 != 0) { fTemp20 } else { fTemp19 } };
			let mut fTemp21: F32 = (1.0 - self.fRec17[0]);
			self.fRec20[0] = ((self.fConst1 * (fTemp0 * self.fRec20[1])) + (fSlow13 * fTemp1));
			let mut fTemp22: F32 = (*input0 as F32);
			let mut fTemp23: F32 = (self.fRec21[1] + 1.0);
			self.fRec22[0] = ((self.fConst1 * (fTemp0 * self.fRec22[1])) + (fSlow17 * fTemp1));
			let mut fTemp24: F32 = (fSlow16 + ((0.100000001 * self.fRec22[0]) + -69.0));
			let mut fTemp25: F32 = ((((fSlow15 * fTemp24) == 0.0) as i32) as F32);
			let mut fTemp26: F32 = (self.fRec21[1] + -1.0);
			self.fRec21[0] = if (((fTemp23 < fTemp25) as i32) as i32 != 0) { fTemp23 } else { if (((fTemp26 > fTemp25) as i32) as i32 != 0) { fTemp26 } else { fTemp25 } };
			let mut fTemp27: F32 = (1.0 - self.fRec21[0]);
			let mut fTemp28: F32 = (fTemp22 * fTemp27);
			self.fVec2[(self.IOTA & 131071) as usize] = fTemp28;
			self.fRec23[0] = libm::fmodf((self.fRec23[1] + (1001.0 - F32::powf(2.0, (fSlow18 * fTemp24)))), 1000.0);
			let mut iTemp29: i32 = (self.fRec23[0] as i32);
			let mut iTemp30: i32 = std::cmp::min(65537, (std::cmp::max(0, (iTemp29 as i32)) as i32));
			let mut fTemp31: F32 = F32::floor(self.fRec23[0]);
			let mut fTemp32: F32 = (fTemp31 + (1.0 - self.fRec23[0]));
			let mut fTemp33: F32 = (self.fRec23[0] - fTemp31);
			let mut iTemp34: i32 = std::cmp::min(65537, (std::cmp::max(0, ((iTemp29 + 1) as i32)) as i32));
			let mut fTemp35: F32 = F32::min((0.100000001 * self.fRec23[0]), 1.0);
			let mut fTemp36: F32 = (self.fRec23[0] + 1000.0);
			let mut iTemp37: i32 = (fTemp36 as i32);
			let mut iTemp38: i32 = std::cmp::min(65537, (std::cmp::max(0, (iTemp37 as i32)) as i32));
			let mut fTemp39: F32 = F32::floor(fTemp36);
			let mut fTemp40: F32 = (fTemp39 + (-999.0 - self.fRec23[0]));
			let mut iTemp41: i32 = std::cmp::min(65537, (std::cmp::max(0, ((iTemp37 + 1) as i32)) as i32));
			let mut fTemp42: F32 = (self.fRec23[0] + (1000.0 - fTemp39));
			let mut fTemp43: F32 = (1.0 - fTemp35);
			let mut fTemp44: F32 = if (iSlow19 as i32 != 0) { fTemp6 } else { 1.0 };
			self.fRec24[0] = ((self.fConst1 * (fTemp0 * self.fRec24[1])) + (fSlow20 * fTemp1));
			let mut fTemp45: F32 = (self.fRec25[1] + 1.0);
			let mut fTemp46: F32 = (self.fRec25[1] + -1.0);
			self.fRec25[0] = if (((fTemp45 < fSlow22) as i32) as i32 != 0) { fTemp45 } else { if (((fTemp46 > fSlow22) as i32) as i32 != 0) { fTemp46 } else { fSlow22 } };
			let mut fTemp47: F32 = (self.fRec26[1] + 1.0);
			let mut fTemp48: F32 = (self.fRec26[1] + -1.0);
			self.fRec26[0] = if (((fTemp47 < fSlow23) as i32) as i32 != 0) { fTemp47 } else { if (((fTemp48 > fSlow23) as i32) as i32 != 0) { fTemp48 } else { fSlow23 } };
			let mut fTemp49: F32 = (self.fRec27[1] + 1.0);
			let mut fTemp50: F32 = (self.fRec27[1] + -1.0);
			self.fRec27[0] = if (((fTemp49 < fSlow24) as i32) as i32 != 0) { fTemp49 } else { if (((fTemp50 > fSlow24) as i32) as i32 != 0) { fTemp50 } else { fSlow24 } };
			self.fRec30[0] = ((self.fConst1 * (fTemp0 * self.fRec30[1])) + (fSlow25 * fTemp1));
			let mut fTemp51: F32 = F32::powf(2.0, (0.0833333358 * (fSlow16 + ((0.00999999978 * self.fRec30[0]) + -69.0))));
			let mut fTemp52: F32 = (self.fRec29[1] + (self.fConst4 * fTemp51));
			self.fRec29[0] = (fTemp52 - F32::floor(fTemp52));
			let mut fTemp53: F32 = (self.iVec0[1] as F32);
			let mut fTemp54: F32 = (440.0 * fTemp51);
			let mut fTemp55: F32 = F32::max(fTemp54, 23.4489498);
			let mut fTemp56: F32 = F32::max(20.0, F32::abs(fTemp55));
			let mut fTemp57: F32 = (self.fRec32[1] + (self.fConst7 * fTemp56));
			self.fRec32[0] = (fTemp57 - F32::floor(fTemp57));
			let mut fTemp58: F32 = insert_faustpower2_f(((2.0 * self.fRec32[0]) + -1.0));
			self.fVec4[0] = fTemp58;
			let mut fTemp59: F32 = ((fTemp53 * (fTemp58 - self.fVec4[1])) / fTemp56);
			self.fVec5[(self.IOTA & 4095) as usize] = fTemp59;
			let mut fTemp60: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst8 / fTemp55)));
			let mut iTemp61: i32 = (fTemp60 as i32);
			let mut fTemp62: F32 = F32::floor(fTemp60);
			let mut fTemp63: F32 = ((fTemp59 - (self.fVec5[((self.IOTA - iTemp61) & 4095) as usize] * (fTemp62 + (1.0 - fTemp60)))) - ((fTemp60 - fTemp62) * self.fVec5[((self.IOTA - (iTemp61 + 1)) & 4095) as usize]));
			self.fRec31[0] = ((0.999000013 * self.fRec31[1]) + (self.fConst6 * fTemp63));
			let mut fTemp64: F32 = F32::max(1.1920929e-07, F32::abs(fTemp54));
			let mut fTemp65: F32 = (self.fRec33[1] + (self.fConst7 * fTemp64));
			let mut fTemp66: F32 = (fTemp65 + -1.0);
			let mut iTemp67: i32 = ((fTemp66 < 0.0) as i32);
			self.fRec33[0] = if (iTemp67 as i32 != 0) { fTemp65 } else { fTemp66 };
			let mut fRec34: F32 = if (iTemp67 as i32 != 0) { fTemp65 } else { (fTemp65 + (fTemp66 * (1.0 - (self.fConst0 / fTemp64)))) };
			let mut fTemp68: F32 = (self.fRec35[1] + 1.0);
			let mut fTemp69: F32 = (self.fRec35[1] + -1.0);
			self.fRec35[0] = if (((fTemp68 < fSlow26) as i32) as i32 != 0) { fTemp68 } else { if (((fTemp69 > fSlow26) as i32) as i32 != 0) { fTemp69 } else { fSlow26 } };
			self.iRec36[0] = ((1103515245 * self.iRec36[1]) + 12345);
			let mut fTemp70: F32 = (self.iRec36[0] as F32);
			self.fRec37[0] = ((self.fConst1 * (fTemp0 * self.fRec37[1])) + (fSlow27 * fTemp1));
			let mut fTemp71: F32 = (self.fRec38[1] + 1.0);
			let mut fTemp72: F32 = (self.fRec38[1] + -1.0);
			self.fRec38[0] = if (((fTemp71 < fSlow29) as i32) as i32 != 0) { fTemp71 } else { if (((fTemp72 > fSlow29) as i32) as i32 != 0) { fTemp72 } else { fSlow29 } };
			let mut fTemp73: F32 = (self.fRec39[1] + 1.0);
			let mut fTemp74: F32 = (self.fRec39[1] + -1.0);
			self.fRec39[0] = if (((fTemp73 < fSlow30) as i32) as i32 != 0) { fTemp73 } else { if (((fTemp74 > fSlow30) as i32) as i32 != 0) { fTemp74 } else { fSlow30 } };
			let mut fTemp75: F32 = (self.fRec40[1] + 1.0);
			let mut fTemp76: F32 = (self.fRec40[1] + -1.0);
			self.fRec40[0] = if (((fTemp75 < fSlow31) as i32) as i32 != 0) { fTemp75 } else { if (((fTemp76 > fSlow31) as i32) as i32 != 0) { fTemp76 } else { fSlow31 } };
			self.fRec42[0] = ((self.fConst1 * (fTemp0 * self.fRec42[1])) + (fSlow32 * fTemp1));
			let mut fTemp77: F32 = F32::powf(2.0, (0.0833333358 * (fSlow16 + ((0.00999999978 * self.fRec42[0]) + -69.0))));
			let mut fTemp78: F32 = (self.fRec41[1] + (self.fConst4 * fTemp77));
			self.fRec41[0] = (fTemp78 - F32::floor(fTemp78));
			let mut fTemp79: F32 = (440.0 * fTemp77);
			let mut fTemp80: F32 = F32::max(fTemp79, 23.4489498);
			let mut fTemp81: F32 = F32::max(20.0, F32::abs(fTemp80));
			let mut fTemp82: F32 = (self.fRec44[1] + (self.fConst7 * fTemp81));
			self.fRec44[0] = (fTemp82 - F32::floor(fTemp82));
			let mut fTemp83: F32 = insert_faustpower2_f(((2.0 * self.fRec44[0]) + -1.0));
			self.fVec6[0] = fTemp83;
			let mut fTemp84: F32 = ((fTemp53 * (fTemp83 - self.fVec6[1])) / fTemp81);
			self.fVec7[(self.IOTA & 4095) as usize] = fTemp84;
			let mut fTemp85: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst8 / fTemp80)));
			let mut iTemp86: i32 = (fTemp85 as i32);
			let mut fTemp87: F32 = F32::floor(fTemp85);
			let mut fTemp88: F32 = ((fTemp84 - (self.fVec7[((self.IOTA - iTemp86) & 4095) as usize] * (fTemp87 + (1.0 - fTemp85)))) - ((fTemp85 - fTemp87) * self.fVec7[((self.IOTA - (iTemp86 + 1)) & 4095) as usize]));
			self.fRec43[0] = ((0.999000013 * self.fRec43[1]) + (self.fConst6 * fTemp88));
			let mut fTemp89: F32 = F32::max(1.1920929e-07, F32::abs(fTemp79));
			let mut fTemp90: F32 = (self.fRec45[1] + (self.fConst7 * fTemp89));
			let mut fTemp91: F32 = (fTemp90 + -1.0);
			let mut iTemp92: i32 = ((fTemp91 < 0.0) as i32);
			self.fRec45[0] = if (iTemp92 as i32 != 0) { fTemp90 } else { fTemp91 };
			let mut fRec46: F32 = if (iTemp92 as i32 != 0) { fTemp90 } else { (fTemp90 + (fTemp91 * (1.0 - (self.fConst0 / fTemp89)))) };
			let mut fTemp93: F32 = (self.fRec47[1] + 1.0);
			let mut fTemp94: F32 = (self.fRec47[1] + -1.0);
			self.fRec47[0] = if (((fTemp93 < fSlow33) as i32) as i32 != 0) { fTemp93 } else { if (((fTemp94 > fSlow33) as i32) as i32 != 0) { fTemp94 } else { fSlow33 } };
			self.fRec48[0] = ((self.fConst1 * (fTemp0 * self.fRec48[1])) + (fSlow34 * fTemp1));
			let mut fTemp95: F32 = (self.fRec49[1] + 1.0);
			let mut fTemp96: F32 = (self.fRec49[1] + -1.0);
			self.fRec49[0] = if (((fTemp95 < fSlow36) as i32) as i32 != 0) { fTemp95 } else { if (((fTemp96 > fSlow36) as i32) as i32 != 0) { fTemp96 } else { fSlow36 } };
			let mut fTemp97: F32 = (self.fRec50[1] + 1.0);
			let mut fTemp98: F32 = (self.fRec50[1] + -1.0);
			self.fRec50[0] = if (((fTemp97 < fSlow37) as i32) as i32 != 0) { fTemp97 } else { if (((fTemp98 > fSlow37) as i32) as i32 != 0) { fTemp98 } else { fSlow37 } };
			let mut fTemp99: F32 = (self.fRec51[1] + 1.0);
			let mut fTemp100: F32 = (self.fRec51[1] + -1.0);
			self.fRec51[0] = if (((fTemp99 < fSlow38) as i32) as i32 != 0) { fTemp99 } else { if (((fTemp100 > fSlow38) as i32) as i32 != 0) { fTemp100 } else { fSlow38 } };
			self.fRec53[0] = ((self.fConst1 * (fTemp0 * self.fRec53[1])) + (fSlow39 * fTemp1));
			let mut fTemp101: F32 = F32::powf(2.0, (0.0833333358 * (fSlow16 + ((0.00999999978 * self.fRec53[0]) + -69.0))));
			let mut fTemp102: F32 = (self.fRec52[1] + (self.fConst4 * fTemp101));
			self.fRec52[0] = (fTemp102 - F32::floor(fTemp102));
			let mut fTemp103: F32 = (440.0 * fTemp101);
			let mut fTemp104: F32 = F32::max(fTemp103, 23.4489498);
			let mut fTemp105: F32 = F32::max(20.0, F32::abs(fTemp104));
			let mut fTemp106: F32 = (self.fRec55[1] + (self.fConst7 * fTemp105));
			self.fRec55[0] = (fTemp106 - F32::floor(fTemp106));
			let mut fTemp107: F32 = insert_faustpower2_f(((2.0 * self.fRec55[0]) + -1.0));
			self.fVec8[0] = fTemp107;
			let mut fTemp108: F32 = ((fTemp53 * (fTemp107 - self.fVec8[1])) / fTemp105);
			self.fVec9[(self.IOTA & 4095) as usize] = fTemp108;
			let mut fTemp109: F32 = F32::max(0.0, F32::min(2047.0, (self.fConst8 / fTemp104)));
			let mut iTemp110: i32 = (fTemp109 as i32);
			let mut fTemp111: F32 = F32::floor(fTemp109);
			let mut fTemp112: F32 = ((fTemp108 - (self.fVec9[((self.IOTA - iTemp110) & 4095) as usize] * (fTemp111 + (1.0 - fTemp109)))) - ((fTemp109 - fTemp111) * self.fVec9[((self.IOTA - (iTemp110 + 1)) & 4095) as usize]));
			self.fRec54[0] = ((0.999000013 * self.fRec54[1]) + (self.fConst6 * fTemp112));
			let mut fTemp113: F32 = F32::max(1.1920929e-07, F32::abs(fTemp103));
			let mut fTemp114: F32 = (self.fRec56[1] + (self.fConst7 * fTemp113));
			let mut fTemp115: F32 = (fTemp114 + -1.0);
			let mut iTemp116: i32 = ((fTemp115 < 0.0) as i32);
			self.fRec56[0] = if (iTemp116 as i32 != 0) { fTemp114 } else { fTemp115 };
			let mut fRec57: F32 = if (iTemp116 as i32 != 0) { fTemp114 } else { (fTemp114 + (fTemp115 * (1.0 - (self.fConst0 / fTemp113)))) };
			let mut fTemp117: F32 = (self.fRec58[1] + 1.0);
			let mut fTemp118: F32 = (self.fRec58[1] + -1.0);
			self.fRec58[0] = if (((fTemp117 < fSlow40) as i32) as i32 != 0) { fTemp117 } else { if (((fTemp118 > fSlow40) as i32) as i32 != 0) { fTemp118 } else { fSlow40 } };
			let mut fTemp119: F32 = (0.00666666683 * (fTemp6 * (((self.fRec24[0] * (((1.0 - self.fRec25[0]) * (((1.0 - self.fRec26[0]) * (((1.0 - self.fRec27[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec29[0]) as i32) as usize] }) + (self.fConst5 * ((self.fRec27[0] * self.fRec31[0]) * fTemp51)))) + (0.5 * (self.fRec26[0] * ((2.0 * fRec34) + -1.0))))) + (self.fRec25[0] * ((self.fConst9 * ((1.0 - self.fRec35[0]) * fTemp63)) + (1.16415322e-10 * (self.fRec35[0] * fTemp70)))))) + (self.fRec37[0] * (((1.0 - self.fRec38[0]) * (((1.0 - self.fRec39[0]) * (((1.0 - self.fRec40[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec41[0]) as i32) as usize] }) + (self.fConst5 * ((self.fRec40[0] * self.fRec43[0]) * fTemp77)))) + (0.5 * (self.fRec39[0] * ((2.0 * fRec46) + -1.0))))) + (self.fRec38[0] * ((self.fConst9 * ((1.0 - self.fRec47[0]) * fTemp88)) + (1.16415322e-10 * (self.fRec47[0] * fTemp70))))))) + (self.fRec48[0] * (((1.0 - self.fRec49[0]) * (((1.0 - self.fRec50[0]) * (((1.0 - self.fRec51[0]) * unsafe { ftbl0insertSIG0[((65536.0 * self.fRec52[0]) as i32) as usize] }) + (self.fConst5 * ((self.fRec51[0] * self.fRec54[0]) * fTemp101)))) + (0.5 * (self.fRec50[0] * ((2.0 * fRec57) + -1.0))))) + (self.fRec49[0] * ((self.fConst9 * ((1.0 - self.fRec58[0]) * fTemp112)) + (1.16415322e-10 * (self.fRec58[0] * fTemp70)))))))));
			let mut fTemp120: F32 = ((0.0399999991 * ((self.fRec20[0] * ((fTemp22 * self.fRec21[0]) + (fTemp27 * ((((self.fVec2[((self.IOTA - iTemp30) & 131071) as usize] * fTemp32) + (fTemp33 * self.fVec2[((self.IOTA - iTemp34) & 131071) as usize])) * fTemp35) + (((self.fVec2[((self.IOTA - iTemp38) & 131071) as usize] * fTemp40) + (self.fVec2[((self.IOTA - iTemp41) & 131071) as usize] * fTemp42)) * fTemp43))))) * fTemp44)) + fTemp119);
			let mut fTemp121: F32 = (3.14159274 * F32::max(0.00999999978, F32::min((self.fConst3 * F32::powf(2.0, (0.0833333358 * ((2.0 * self.fRec18[0]) + -79.0)))), 0.99000001)));
			let mut fTemp122: F32 = F32::cos(fTemp121);
			let mut fTemp123: F32 = (0.0 - (2.0 * fTemp122));
			self.fRec59[0] = ((self.fConst1 * (fTemp0 * self.fRec59[1])) + (fSlow41 * fTemp1));
			let mut fTemp124: F32 = (0.5 * (F32::sin(fTemp121) * F32::powf(10.0, (0.0 - (0.0500000007 * self.fRec59[0])))));
			let mut fTemp125: F32 = (1.0 - fTemp124);
			let mut fTemp126: F32 = (fTemp124 + 1.0);
			self.fRec19[0] = ((fTemp21 * fTemp120) - (((self.fRec19[1] * fTemp123) + (self.fRec19[2] * fTemp125)) / fTemp126));
			let mut fTemp127: F32 = (-1.0 - fTemp122);
			let mut fTemp128: F32 = (0.0 - (0.5 * fTemp127));
			let mut fTemp129: F32 = (((fTemp21 * (((self.fRec19[0] * fTemp128) + (self.fRec19[1] * fTemp127)) + (self.fRec19[2] * fTemp128))) / fTemp126) + (self.fRec17[0] * fTemp120));
			let mut fTemp130: F32 = (3.14159274 * F32::max(0.00999999978, F32::min((self.fConst10 * ((0.5 * self.fRec14[0]) + 60.0)), 0.99000001)));
			let mut fTemp131: F32 = (0.0 - (2.0 * F32::cos(fTemp130)));
			let mut fTemp132: F32 = (self.fRec16[1] * fTemp131);
			let mut fTemp133: F32 = F32::sin(fTemp130);
			let mut fTemp134: F32 = F32::powf(10.0, (0.0125000002 * self.fRec15[0]));
			let mut fTemp135: F32 = (0.125 * (fTemp133 / fTemp134));
			let mut fTemp136: F32 = (1.0 - fTemp135);
			let mut fTemp137: F32 = (fTemp135 + 1.0);
			self.fRec16[0] = ((fTemp17 * fTemp129) - ((fTemp132 + (self.fRec16[2] * fTemp136)) / fTemp137));
			let mut fTemp138: F32 = (0.125 * (fTemp133 * fTemp134));
			let mut fTemp139: F32 = (fTemp138 + 1.0);
			let mut fTemp140: F32 = (1.0 - fTemp138);
			let mut fTemp141: F32 = (((fTemp17 * ((fTemp132 + (self.fRec16[0] * fTemp139)) + (self.fRec16[2] * fTemp140))) / fTemp137) + (self.fRec13[0] * fTemp129));
			let mut fTemp142: F32 = (0.0 - (2.0 * fTemp12));
			self.fRec60[0] = ((self.fConst1 * (fTemp0 * self.fRec60[1])) + (fSlow42 * fTemp1));
			let mut fTemp143: F32 = (0.5 * (F32::sin(fTemp11) * F32::powf(10.0, (0.0 - (0.0500000007 * self.fRec60[0])))));
			let mut fTemp144: F32 = (1.0 - fTemp143);
			let mut fTemp145: F32 = (fTemp143 + 1.0);
			self.fRec12[0] = ((fTemp10 * fTemp141) - (((self.fRec12[1] * fTemp142) + (self.fRec12[2] * fTemp144)) / fTemp145));
			let mut fTemp146: F32 = (((fTemp13 * ((self.fRec12[1] + (0.5 * self.fRec12[0])) + (0.5 * self.fRec12[2]))) / fTemp145) + (self.fRec3[0] * fTemp141));
			let mut fTemp147: F32 = (*input1 as F32);
			let mut fTemp148: F32 = (fTemp147 * fTemp27);
			self.fVec10[(self.IOTA & 131071) as usize] = fTemp148;
			let mut fTemp149: F32 = (fTemp119 + (0.0399999991 * ((self.fRec20[0] * fTemp44) * ((fTemp147 * self.fRec21[0]) + (fTemp27 * ((fTemp35 * ((self.fVec10[((self.IOTA - iTemp30) & 131071) as usize] * fTemp32) + (fTemp33 * self.fVec10[((self.IOTA - iTemp34) & 131071) as usize]))) + (fTemp43 * ((self.fVec10[((self.IOTA - iTemp38) & 131071) as usize] * fTemp40) + (self.fVec10[((self.IOTA - iTemp41) & 131071) as usize] * fTemp42)))))))));
			self.fRec63[0] = ((fTemp21 * fTemp149) - (((fTemp123 * self.fRec63[1]) + (fTemp125 * self.fRec63[2])) / fTemp126));
			let mut fTemp150: F32 = (((fTemp21 * (((self.fRec63[0] * fTemp128) + (fTemp127 * self.fRec63[1])) + (fTemp128 * self.fRec63[2]))) / fTemp126) + (self.fRec17[0] * fTemp149));
			let mut fTemp151: F32 = (fTemp131 * self.fRec62[1]);
			self.fRec62[0] = ((fTemp17 * fTemp150) - ((fTemp151 + (fTemp136 * self.fRec62[2])) / fTemp137));
			let mut fTemp152: F32 = (((fTemp17 * ((fTemp151 + (self.fRec62[0] * fTemp139)) + (fTemp140 * self.fRec62[2]))) / fTemp137) + (self.fRec13[0] * fTemp150));
			self.fRec61[0] = ((fTemp10 * fTemp152) - (((fTemp142 * self.fRec61[1]) + (fTemp144 * self.fRec61[2])) / fTemp145));
			let mut fTemp153: F32 = (((fTemp13 * ((self.fRec61[1] + (0.5 * self.fRec61[0])) + (0.5 * self.fRec61[2]))) / fTemp145) + (self.fRec3[0] * fTemp152));
			let mut fTemp154: F32 = (0.0399999991 * self.fRec2[0]);
			let mut fTemp155: F32 = if (iTemp2 as i32 != 0) { (fTemp146 * (1.0 - fTemp154)) } else { (fTemp146 + (fTemp153 * F32::abs(fTemp154))) };
			*output0 = ((0.0199999996 * (self.fRec0[0] * fTemp155)) as F32);
			let mut fTemp156: F32 = if (iTemp2 as i32 != 0) { (fTemp153 + (0.0399999991 * (self.fRec2[0] * fTemp146))) } else { (fTemp153 * (fTemp154 + 1.0)) };
			*output1 = ((0.0199999996 * (self.fRec0[0] * fTemp156)) as F32);
			self.fRec64[0] = ((self.fConst1 * (fTemp0 * self.fRec64[1])) + (fSlow43 * fTemp1));
			*output2 = ((0.0199999996 * (self.fRec64[0] * fTemp155)) as F32);
			*output3 = ((0.0199999996 * (self.fRec64[0] * fTemp156)) as F32);
			self.fRec65[0] = ((self.fConst1 * (fTemp0 * self.fRec65[1])) + (fSlow44 * fTemp1));
			*output4 = ((0.0199999996 * (self.fRec65[0] * fTemp155)) as F32);
			*output5 = ((0.0199999996 * (self.fRec65[0] * fTemp156)) as F32);
			self.fRec66[0] = ((self.fConst1 * (fTemp0 * self.fRec66[1])) + (fSlow45 * fTemp1));
			*output6 = ((0.0199999996 * (self.fRec66[0] * fTemp155)) as F32);
			*output7 = ((0.0199999996 * (self.fRec66[0] * fTemp156)) as F32);
			self.iVec0[1] = self.iVec0[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec9[1] = self.fRec9[0];
			self.iRec10[1] = self.iRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec21[1] = self.fRec21[0];
			self.IOTA = (self.IOTA + 1);
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec32[1] = self.fRec32[0];
			self.fVec4[1] = self.fVec4[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec35[1] = self.fRec35[0];
			self.iRec36[1] = self.iRec36[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec44[1] = self.fRec44[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec55[1] = self.fRec55[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec19[2] = self.fRec19[1];
			self.fRec19[1] = self.fRec19[0];
			self.fRec16[2] = self.fRec16[1];
			self.fRec16[1] = self.fRec16[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fRec63[2] = self.fRec63[1];
			self.fRec63[1] = self.fRec63[0];
			self.fRec62[2] = self.fRec62[1];
			self.fRec62[1] = self.fRec62[0];
			self.fRec61[2] = self.fRec61[1];
			self.fRec61[1] = self.fRec61[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec66[1] = self.fRec66[0];
		}
	}

}

