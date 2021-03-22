struct Sequence {
    struct Step {
        bool active;
        bool skipNext;
        int note = 69;
    };

    enum View {
        View_none,
        View_empty,
        View_exactlyOnStep,
        View_containsSteps,
    };

    static const int perPage = 4;
    static const int maxResolution = 64;

    int resolution = 4;

    int bars() {
        return std::ceil(1.f * length / maxResolution);
    }

    int view(int i) {
        return viewFrom(viewIndexToStart(i));
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
        auto start = viewIndexToStart(i);
        switch (viewFrom(start)) {
            case View_none:
                return;
            case View_empty:
            case View_exactlyOnStep:
                steps[start].active ^= true;
                steps[start].skipNext = false;
                return;
            case View_containsSteps:
                for (int i = start; i < start + viewLength(); i++)
                    steps[i].active = false;
                return;
        }
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

    View viewFrom(int start) {
        if (start >= length)
            return View_none;
        int countActive = 0;
        int lastActive = 0;
        for (int i = start; i < start + viewLength(); i++) {
            if (steps[i].active) {
                countActive++;
                lastActive = i;
            }
        }
        if (countActive == 0)
            return View_empty;
        else if (countActive == 1 && lastActive == start)
            return View_exactlyOnStep;
        else
            return View_containsSteps;
    }

    std::array<Step, maxResolution*16*8> steps;
};
