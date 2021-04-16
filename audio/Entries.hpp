struct Entries : GenericUI {
    struct Control {
        const char* label;
        float value;
        float min;
        float max;
        float step;
    };

    std::vector<Control> data;

    Entries() : data() {
    }

    /*
     * Since the order of calls to `addNumEntry` will be stable,
     * we can use a simple vector to power our dynamic voice assigmnent.
     * When `buildUserInterface` is called initially, we append each new control
     * into the `data` vector. After a call to `prepareToWrite` however,
     * instead of pushing _to_ the array, we read _from_ it. On subsequent calls
     * to `buildUserInterface`, the calling dsp has its zones set to reflect
     * the values in `data`.
     */
    void addNumEntry(const char* label, FAUSTFLOAT* zone, FAUSTFLOAT init, FAUSTFLOAT min, FAUSTFLOAT max, FAUSTFLOAT step) override {
        if (writeIndex < 0)
            data.push_back({ label, init, min, max, step });
        else
            *zone = data[writeIndex++].value;
    }

    void prepareToWrite() {
        writeIndex = 0;
    }

  private:
    int writeIndex = -1;
};
