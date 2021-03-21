struct Sequence {
    struct Step {
        bool active;
        bool skipNext;
        int note = 69;
    };

    static const int perPage = 4;
    static const int maxResolution = 64;

    int resolution = 4;

    int bars() {
        return std::ceil(1.f * length / maxResolution);
    }

    int view(int i) {
        auto start = viewIndexToStart(i);
        if (start >= length)
            return 0;
        else if (!steps[start].active)
            return 1;
        else
            return 2;
    }

    int viewStart() {
        return pageStart / viewLength();
    }

    void movePage(int diff) {
        auto newPageStart = pageStart + diff * pageLength();
        if (newPageStart < length)
            pageStart = (std::max)(0, newPageStart);
    }

    void zoomOut() {
        if (resolution > 1) {
            resolution /= 2;
            pageStart = rescale(pageStart);
        }
    }

    void zoomIn() {
        if (resolution < maxResolution)
            resolution *= 2;
    }

    void toggle(int i) {
        auto index = viewIndexToStart(i);
        steps[index].active ^= true;
        steps[index].skipNext = false;
    }

    void record(int position, int note) {
        auto quantizedPosition = rescale(position + viewLength()/2);
        auto quantizedIndex = quantizedPosition % length;
        steps[quantizedIndex].active = true;
        steps[quantizedIndex].skipNext = quantizedPosition > position;
        steps[quantizedIndex].note = note;
    }

    bool at(int position, int& note) {
        auto& step = steps[position % length];
        if (step.skipNext) {
            step.skipNext = false;
            return false;
        }
        note = step.note;
        return step.active;
    }

  private:
    int pageStart = 0;
    int length = maxResolution*4;

    int rescale(int position) {
        return position / viewLength() * viewLength();
    }

    int pageLength() {
        return perPage * viewLength();
    }

    int viewLength() {
        return maxResolution / resolution;
    }

    int viewIndexToStart(int i) {
        return pageStart + i * viewLength();
    }

    std::array<Step, maxResolution*16*8> steps;
};
