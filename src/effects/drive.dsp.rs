
fn drive_faustpower2_f(value: F32) -> F32 {
	return (value * value);
}
pub struct drive {
	fSampleRate: i32,
	fConst0: F32,
	fEntry0: F32,
	fConst1: F32,
	fRec1: [F32;2],
	fEntry1: F32,
	fRec2: [F32;2],
	fEntry2: F32,
	fRec3: [F32;2],
	fVec0: [F32;2],
	fRec0: [F32;2],
	fVec1: [F32;2],
	fRec4: [F32;2],
}

impl FaustDsp for drive {
	type T = F32;
		
	fn new() -> drive { 
		drive {
			fSampleRate: 0,
			fConst0: 0.0,
			fEntry0: 0.0,
			fConst1: 0.0,
			fRec1: [0.0;2],
			fEntry1: 0.0,
			fRec2: [0.0;2],
			fEntry2: 0.0,
			fRec3: [0.0;2],
			fVec0: [0.0;2],
			fRec0: [0.0;2],
			fVec1: [0.0;2],
			fRec4: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("filename", "drive.dsp");
		m.declare("filters.lib/dcblocker:author", "Julius O. Smith III");
		m.declare("filters.lib/dcblocker:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/dcblocker:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/pole:author", "Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", "MIT-style STK-4.3 license");
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
		m.declare("name", "drive");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.0");
		m.declare("spats.lib/name", "Faust Spatialization Library");
		m.declare("spats.lib/version", "0.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 2;
	}
	fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fEntry0 = 15.0;
		self.fEntry1 = 25.0;
		self.fEntry2 = 25.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec1[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec2[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec3[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fVec0[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec0[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fVec1[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec4[l6 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = (44.0999985 / F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32))));
		self.fConst1 = (1.0 - self.fConst0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		drive::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("drive");
		ui_interface.add_num_entry("driveFeed", ParamIndex(0), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("driveGain", ParamIndex(1), 25.0, 0.0, 50.0, 10.0);
		ui_interface.add_num_entry("driveSpace", ParamIndex(2), 15.0, 0.0, 50.0, 10.0);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			2 => Some(self.fEntry0),
			1 => Some(self.fEntry1),
			0 => Some(self.fEntry2),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			2 => { self.fEntry0 = value }
			1 => { self.fEntry1 = value }
			0 => { self.fEntry2 = value }
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
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = (self.fConst0 * (self.fEntry0 as F32));
		let mut fSlow1: F32 = (self.fConst0 * (self.fEntry1 as F32));
		let mut fSlow2: F32 = (self.fConst0 * (self.fEntry2 as F32));
		let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
		for (((input0, input1), output0), output1) in zipped_iterators {
			self.fRec1[0] = (fSlow0 + (self.fConst1 * self.fRec1[1]));
			self.fRec2[0] = (fSlow1 + (self.fConst1 * self.fRec2[1]));
			self.fRec3[0] = (fSlow2 + (self.fConst1 * self.fRec3[1]));
			let mut fTemp0: F32 = F32::powf(10.0, (0.0399999991 * self.fRec3[0]));
			let mut fTemp1: F32 = F32::max(-1.0, F32::min(1.0, (0.0199999996 * (self.fRec1[0] + (((*input0 as F32) * self.fRec2[0]) * fTemp0)))));
			let mut fTemp2: F32 = (fTemp1 * (1.0 - (0.333333343 * drive_faustpower2_f(fTemp1))));
			self.fVec0[0] = fTemp2;
			self.fRec0[0] = (((0.995000005 * self.fRec0[1]) + fTemp2) - self.fVec0[1]);
			*output0 = (self.fRec0[0] as F32);
			let mut fTemp3: F32 = F32::max(-1.0, F32::min(1.0, (0.0199999996 * (self.fRec1[0] + (((*input1 as F32) * self.fRec2[0]) * fTemp0)))));
			let mut fTemp4: F32 = (fTemp3 * (1.0 - (0.333333343 * drive_faustpower2_f(fTemp3))));
			self.fVec1[0] = fTemp4;
			self.fRec4[0] = (((0.995000005 * self.fRec4[1]) + fTemp4) - self.fVec1[1]);
			*output1 = (self.fRec4[0] as F32);
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec3[1] = self.fRec3[0];
			self.fVec0[1] = self.fVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec4[1] = self.fRec4[0];
		}
	}

}

