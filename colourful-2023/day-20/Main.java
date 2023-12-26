import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.atomic.AtomicLong;

public class Main {
	private static final HashMap<String, Module> moduleMap = new HashMap<>();
	private static final HashMap<String, List<String>> connectionMap = new HashMap<>();

	public static void main(String[] args) throws Exception {
		String input = Files.readString(Paths.get("./input.txt"));
		parseModules(input);

		// long res1 = pushButtonManyTimes();
		// System.err.println(res1);

		BigInteger res2 = calculateLowToRx();
		System.err.println(res2);
	}

	private static void parseModules(String input) {
		Arrays.stream(input.split("\n")).forEach(line -> processLine(line.trim()));

		connectionMap.forEach((key, value) -> {
			Module fromModule = moduleMap.get(key);
			value.forEach(outModuleName -> {
				moduleMap.computeIfAbsent(outModuleName, Broadcaster::new);
				Module toModule = moduleMap.get(outModuleName);
				fromModule.addOutgoing(toModule);
				toModule.addIncoming(fromModule);
			});
		});
	}

	private static void processLine(String line) {
		String[] parts = line.split(" -> ");
		String moduleName = parts[0];
		String[] connectedModules = parts[1].split(", ");

		Module module;
		if (moduleName.equals("broadcaster")) {
			module = new Broadcaster(moduleName);
		} else if (moduleName.startsWith("%")) {
			moduleName = moduleName.substring(1);
			module = new FlipFlop(moduleName);
		} else {
			moduleName = moduleName.substring(1);
			module = new Conjunction(moduleName);
		}

		moduleMap.put(moduleName, module);
		connectionMap.put(moduleName, Arrays.asList(connectedModules));
	}

	private static long pushButtonManyTimes() {
		Counter counter = new Counter();
		for (int i = 0; i < 1000; i++) {
			pushOnce(counter);
		}
		return counter.high * counter.low;
	}

	private static BigInteger calculateLowToRx() {
		Module rxModule = moduleMap.get("rx");
		if (!(rxModule instanceof Broadcaster)) {
			throw new IllegalStateException("Rx module is not a Broadcaster.");
		}

		Module prevModule = ((Broadcaster) rxModule).incoming;
		if (!(prevModule instanceof Conjunction)) {
			throw new IllegalStateException("Previous module is not a Conjunction.");
		}

		Set<String> repeaters = ((Conjunction) prevModule).states.keySet();
		AtomicLong turn = new AtomicLong(1);
		Map<String, Long> cycles = new HashMap<>();

		ICounter counter = state -> {
			if (state.from() != null && repeaters.contains(state.from().name) && state.pulse() == Pulse.High) {
				cycles.putIfAbsent(state.from().name, turn.get());
			}
		};

		while (cycles.size() != repeaters.size()) {
			pushOnce(counter);
			turn.incrementAndGet();
		}

		return cycles.values().stream()
				.map(BigInteger::valueOf)
				.reduce(BigInteger.ONE, (a, b) -> a.multiply(b.divide(a.gcd(b))));
	}

	private static void pushOnce(ICounter counter) {
		LinkedList<State> queue = new LinkedList<>();
		queue.add(new State(null, moduleMap.get("broadcaster"), Pulse.Low));

		while (!queue.isEmpty()) {
			State current = queue.poll();
			counter.count(current);
			current.to().receive(current.from(), current.pulse(), queue);
		}
	}
}

enum Pulse {
	High, Low
}

abstract class Module {
	protected List<Module> out = new ArrayList<>();
	public String name;

	public Module(String name) {
		this.name = name;
	}

	public void addIncoming(Module from) {
	}

	public void addOutgoing(Module m) {
		out.add(m);
	}

	protected final void send(Pulse pulse, List<State> queue) {
		for (Module m : out) {
			queue.add(new State(this, m, pulse));
		}
	}

	public abstract void receive(Module src, Pulse pulse, List<State> queue);
}

class FlipFlop extends Module {
	boolean on = false;

	public FlipFlop(String name) {
		super(name);
	}

	@Override
	public void receive(Module src, Pulse pulse, List<State> queue) {
		if (pulse == Pulse.Low) {
			on = !on;
			send(on ? Pulse.High : Pulse.Low, queue);
		}
	}
}

class Conjunction extends Module {
	HashMap<String, Pulse> states = new HashMap<>();

	public Conjunction(String name) {
		super(name);
	}

	@Override
	public void addIncoming(Module from) {
		states.put(from.name, Pulse.Low);
	}

	@Override
	public void receive(Module src, Pulse pulse, List<State> queue) {
		states.put(src.name, pulse);
		Pulse send = Pulse.Low;
		for (var elem : states.values()) {
			if (elem == Pulse.Low) {
				send = Pulse.High;
				break;
			}
		}
		send(send, queue);
	}
}

class Broadcaster extends Module {
	public Module incoming;

	public Broadcaster(String name) {
		super(name);
	}

	@Override
	public void addIncoming(Module from) {
		if (incoming != null) {
			throw new IllegalStateException();
		}
		incoming = from;
	}

	@Override
	public void receive(Module src, Pulse pulse, List<State> queue) {
		send(pulse, queue);
	}
}

record State(Module from, Module to, Pulse pulse) {
}

@FunctionalInterface
interface ICounter {
	public void count(State state);
}

class Counter implements ICounter {
	public long low;
	public long high;

	public void count(State state) {
		if (state.pulse() == Pulse.Low) {
			low += 1;
		} else {
			high += 1;
		}
	}
}