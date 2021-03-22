struct one_sample_dsp : dsp {
    std::unique_ptr<Destinations> destinations;

    void prepare() {
        init(SAMPLE_RATE);
        intControls = std::unique_ptr<int[]>(new int[getNumIntControls()]);
        floatControls = std::unique_ptr<float[]>(new float[getNumRealControls()]);
        destinations = std::make_unique<Destinations>();
        UI ui = { destinations.get() };
        buildUserInterface(&ui);
    }

    void render(FAUSTFLOAT* inputs, FAUSTFLOAT* outputs) {
        control(intControls.get(), floatControls.get());
        compute(inputs, outputs, intControls.get(), floatControls.get());
    }

private:
    std::unique_ptr<int[]> intControls;
    std::unique_ptr<float[]> floatControls;

    virtual int getNumIntControls() = 0;
    virtual int getNumRealControls() = 0;
    virtual void control(int* iControl, FAUSTFLOAT* fControl) = 0;
    virtual void compute(FAUSTFLOAT* inputs, FAUSTFLOAT* outputs, int* iControl, FAUSTFLOAT* fControl) = 0;

    void compute(int, FAUSTFLOAT**, FAUSTFLOAT**) override {}
};
